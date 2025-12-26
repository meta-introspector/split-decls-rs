//! This module defines core data structures (`Invocation`, `InvocationKind`, and `ExpansionData`)
//! that represent macro invocations and their context within the Rust compiler's expansion engine.
//! These are fundamental for tracking and processing how macros are called and expanded in the AST.
//!
//! For n00bs: Imagine your code is a script, and macros are like special instructions or spells.
//!
//! - `Invocation`: This is the spell itself. It describes a single instance of a macro being called
//!   in your code. It holds details like *what kind* of spell it is (`InvocationKind`) and *where*
//!   it's being cast in the story (`ExpansionData`).
//!
//! - `InvocationKind`: This tells you what *type* of spell it is. Is it a loud "bang" spell (`foo!()`),
//!   a subtle "attribute" charm (`#[attr]`), a powerful "derive" ritual (`#[derive()]`), or a special
//!   "glob delegation" that delegates powers? This enum categorizes the different ways you can use macros.
//!
//! - `ExpansionData`: This is all the "background information" about where and when a spell is being
//!   cast. It includes a unique ID for this specific casting (`LocalExpnId`), how deep you are in a
//!   sequence of spells (macros calling other macros), which module the spell is happening in, and
//!   other contextual details vital for the compiler to understand the macro's environment.
//!
//! Moving these structures to `rustc_expand_base_lib` means that any part of the compiler that needs
//! to understand or interact with macro calls can rely on these centralized definitions. It creates
//! a common language for discussing and handling macro invocations across different compiler components.

use std::rc::Rc;
use std::path::PathBuf;

use rustc_ast::{self as ast, HasAttrs, HasTokens, NodeId};
use rustc_span::{Ident, LocalExpnId, Span};
use rustc_span::hygiene::{ExpnData as HygieneExpnData, ExpnKind, MacroKind};

use crate::prelude::{Annotatable, AstFragmentKind};

/// Represents a single macro invocation found in the AST.
///
/// This struct encapsulates all the necessary information about a macro call
/// that the expansion engine needs to process it.
#[derive(Debug)]
pub struct Invocation {
    /// The specific kind of macro invocation (bang, attribute, derive, glob delegation).
    pub kind: InvocationKind,
    /// The expected kind of AST fragment that this macro should expand into.
    pub fragment_kind: AstFragmentKind,
    /// Contextual data about where and when this macro is being expanded.
    pub expansion_data: ExpansionData,
}

/// Enumerates the different kinds of macro invocations.
///
/// Each variant corresponds to a different syntax and behavior for calling macros.
#[derive(Debug)]
pub enum InvocationKind {
    /// A function-like macro invocation, e.g., `vec![1, 2, 3]`.
    Bang {
        mac: Box<ast::MacCall>,
        span: Span,
    },
    /// An attribute-like macro invocation, e.g., `#[derive(Debug)]`.
    Attr {
        attr: ast::Attribute,
        /// Re-insertion position for inert attributes.
        pos: usize,
        item: Annotatable,
        /// Required for resolving derive helper attributes.
        derives: Vec<ast::Path>,
    },
    /// A `derive` macro invocation, e.g., `#[derive(Debug)]`.
    /// This is distinct from `Attr` because `derive` macros have special handling.
    Derive {
        path: ast::Path,
        is_const: bool,
        item: Annotatable,
    },
    /// A glob delegation macro, used for specialized trait implementations.
    GlobDelegation {
        item: Box<ast::AssocItem>,
        /// Whether this is a trait impl or an inherent impl.
        of_trait: bool,
    },
}

impl InvocationKind {
    /// Determines the placeholder visibility for a given `InvocationKind`.
    ///
    /// This is a heuristic to ensure that when a macro expands, any placeholder
    /// generated has appropriate visibility, particularly for fields in tuple
    /// structs/variants where visibility is determined early in the compilation
    /// process.
    pub fn placeholder_visibility(&self) -> Option<ast::Visibility> {
        // HACK: For unnamed fields placeholders should have the same visibility as the actual
        // fields because for tuple structs/variants resolve determines visibilities of their
        // constructor using these field visibilities before attributes on them are expanded.
        // The assumption is that the attribute expansion cannot change field visibilities,
        // and it holds because only inert attributes are supported in this position.
        match self {
            InvocationKind::Attr { item: Annotatable::FieldDef(field), .. }
            | InvocationKind::Derive { item: Annotatable::FieldDef(field), .. }
                if field.ident.is_none() =>
            {
                Some(field.vis.clone())
            }
            _ => None,
        }
    }
}

impl Invocation {
    /// Returns the `Span` (source code location) of this macro invocation.
    pub fn span(&self) -> Span {
        match &self.kind {
            InvocationKind::Bang { span, .. } => *span,
            InvocationKind::Attr { attr, .. } => attr.span,
            InvocationKind::Derive { path, .. } => path.span,
            InvocationKind::GlobDelegation { item, .. } => item.span,
        }
    }

    /// Returns a mutable reference to the `Span` of this macro invocation.
    pub fn span_mut(&mut self) -> &mut Span {
        match &mut self.kind {
            InvocationKind::Bang { span, .. } => span,
            InvocationKind::Attr { attr, .. } => &mut attr.span,
            InvocationKind::Derive { path, .. } => &mut path.span,
            InvocationKind::GlobDelegation { item, .. } => &mut item.span,
        }
    }
}


/// Contextual data associated with a macro expansion.
///
/// This struct provides a snapshot of the compiler's state at the time of a macro expansion,
/// which is crucial for hygiene, error reporting, and accurate source mapping.
///
/// For n00bs: This is like the "who, what, when, where" of a macro. It keeps track of:
/// - `id`: A unique ID for this specific expansion.
/// - `depth`: How many macros deep we are (macro calling another macro, etc.).
/// - `module`: Information about the module where the macro is being expanded.
/// - `dir_ownership`: How the directory is handled (important for resolving paths).
/// - `lint_node_id`: A reference to a nearby AST node for associating lints.
/// - `is_trailing_mac`: If this is a macro call at the end of a block.
#[derive(Clone, Debug)]
pub struct ExpansionData {
    pub id: LocalExpnId,
    pub depth: usize,
    pub module: Rc<ModuleData>, // ModuleData needs to be moved too
    pub dir_ownership: DirOwnership, // DirOwnership needs to be moved too
    /// Some parent node that is close to this macro call
    pub lint_node_id: NodeId,
    pub is_trailing_mac: bool,
}

// ModuleData needs to be defined for ExpansionData.
// DirOwnership needs to be defined for ExpansionData.

#[derive(Debug, Clone, Default)]
pub struct ModuleData {
    /// Path to the module starting from the crate name, like `my_crate::foo::bar`.
    pub mod_path: Vec<Ident>,
    /// Stack of paths to files loaded by out-of-line module items,
    /// used to detect and report recursive module inclusions.
    pub file_path_stack: Vec<PathBuf>,
    /// Directory to search child module files in,
    /// often (but not necessarily) the parent of the top file path on the `file_path_stack`.
    pub dir_path: PathBuf,
}

impl ModuleData {
    pub fn with_dir_path(&self, dir_path: PathBuf) -> ModuleData {
        ModuleData {
            mod_path: self.mod_path.clone(),
            file_path_stack: self.file_path_stack.clone(),
            dir_path,
        }
    }
}

/// Describes the ownership state of a directory.
///
/// Used primarily for `mod` file resolution.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DirOwnership {
    /// The directory is owned by the current module.
    Owned {
        /// If `Some`, the owning module is an inline module defined at the given path.
        relative: Option<PathBuf>,
    },
    /// The directory is unowned, typically via a `block`.
    UnownedViaBlock,
}
