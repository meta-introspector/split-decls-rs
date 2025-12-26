//! This module defines the `InvocationCollectorNode` trait and related structures, which are
//! crucial for how the Rust compiler's macro expansion engine processes different types of
//! Abstract Syntax Tree (AST) nodes.
//!
//! For n00bs: Imagine the compiler is walking through your code, looking for places where macros
//! are used. The `InvocationCollectorNode` trait tells the compiler, for each different kind
//! of code structure (like a function, a variable, an expression, etc.), "how to behave"
//! when it encounters a macro call within that structure.
//!
//! This trait provides a standardized way for the macro expansion system to:
//! - Identify macro invocations within an AST node.
//! - Extract information about those invocations (like their attributes or associated data).
//! - Replace the original macro call with the expanded code, or a placeholder if expansion fails.
//!
//! By having this trait in `rustc_expand_base_lib`, it becomes a fundamental building block
//! for any part of the compiler that needs to interact with the macro expansion process,
//! ensuring consistent handling of macro calls across various AST nodes.

use rustc_ast::{self as ast, Attribute, AttrVec, HasAttrs, HasNodeId, NodeId, AstNodeWrapper};
use rustc_span::{Ident, Span};
use smallvec::SmallVec;

use crate::expanded_nodes::expanded_stmt::ExpandedStmt; // Import ExpandedStmt from its new location

// Newtype wrappers to overcome the orphan rule for InvocationCollectorNode implementations.
// These wrap Boxed AST nodes or direct AST nodes from `rustc_ast`.
pub struct ExpandedItem(pub Box<ast::Item>);
pub struct ExpandedAssocItem(pub Box<ast::AssocItem>);
pub struct ExpandedForeignItem(pub Box<ast::ForeignItem>);
pub struct ExpandedVariant(pub ast::Variant);
pub struct ExpandedWherePredicate(pub ast::WherePredicate);
pub struct ExpandedFieldDef(pub ast::FieldDef);
pub struct ExpandedPatField(pub ast::PatField);
pub struct ExpandedExprField(pub ast::ExprField);
pub struct ExpandedParam(pub ast::Param);
pub struct ExpandedGenericParam(pub ast::GenericParam);
pub struct ExpandedArm(pub ast::Arm);

pub struct ExpandedCrate(pub ast::Crate);
pub struct ExpandedTy(pub Box<ast::Ty>);
pub struct ExpandedPat(pub Box<ast::Pat>);
pub struct ExpandedExpr(pub Box<ast::Expr>);

impl HasNodeId for ExpandedItem {
    fn node_id(&self) -> NodeId {
        self.0.id
    }
    fn node_id_mut(&mut self) -> &mut NodeId {
        &mut self.0.id
    }
}

impl HasAttrs for ExpandedItem {
    const SUPPORTS_CUSTOM_INNER_ATTRS: bool = false;

    fn attrs(&self) -> &[Attribute] {
        self.0.attrs.as_slice()
    }
    fn visit_attrs(&mut self, f: impl FnOnce(&mut AttrVec)) {
        f(&mut self.0.attrs);
    }
}

impl HasNodeId for ExpandedExpr {
    fn node_id(&self) -> NodeId {
        self.0.id
    }
    fn node_id_mut(&mut self) -> &mut NodeId {
        &mut self.0.id
    }
}

impl HasAttrs for ExpandedExpr {
    const SUPPORTS_CUSTOM_INNER_ATTRS: bool = false;

    fn attrs(&self) -> &[Attribute] {
        self.0.attrs.as_slice()
    }
    fn visit_attrs(&mut self, f: impl FnOnce(&mut AttrVec)) {
        f(&mut self.0.attrs);
    }
}

impl HasNodeId for ExpandedCrate {
    fn node_id(&self) -> NodeId {
        self.0.id
    }
    fn node_id_mut(&mut self) -> &mut NodeId {
        &mut self.0.id
    }
}

impl HasAttrs for ExpandedCrate {
    const SUPPORTS_CUSTOM_INNER_ATTRS: bool = false;

    fn attrs(&self) -> &[Attribute] {
        self.0.attrs.as_slice()
    }
    fn visit_attrs(&mut self, f: impl FnOnce(&mut AttrVec)) {
        f(&mut self.0.attrs);
    }
}

impl HasNodeId for ExpandedTy {
    fn node_id(&self) -> NodeId {
        self.0.id
    }
    fn node_id_mut(&mut self) -> &mut NodeId {
        &mut self.0.id
    }
}

impl HasAttrs for ExpandedTy {
    const SUPPORTS_CUSTOM_INNER_ATTRS: bool = false; // Ty does not directly support inner attributes

    fn attrs(&self) -> &[Attribute] {
        &[] // Ty does not have attributes directly
    }
    fn visit_attrs(&mut self, _f: impl FnOnce(&mut AttrVec)) {
        // Ty does not have attributes, so do nothing
    }
}

impl HasNodeId for ExpandedPat {
    fn node_id(&self) -> NodeId {
        self.0.id
    }
    fn node_id_mut(&mut self) -> &mut NodeId {
        &mut self.0.id
    }
}

impl HasAttrs for ExpandedPat {
    const SUPPORTS_CUSTOM_INNER_ATTRS: bool = false; // Pat does not directly support inner attributes

    fn attrs(&self) -> &[Attribute] {
        &[] // Pat does not have attributes directly
    }
    fn visit_attrs(&mut self, _f: impl FnOnce(&mut AttrVec)) {
        // Pat does not have attributes, so do nothing
    }
}

pub struct ExpandedAstNodeWrapper<T, Tag>(pub ast::AstNodeWrapper<T, Tag>);

// Use types now residing in rustc_expand_base_lib
use crate::prelude::{Annotatable, AddSemicolon};
use crate::cfg_false_reporter::CfgFalseReporter;
use crate::ast_traits::{MacroInvocationNode, AstFragmentConverter, WalkableAstNode, CfgFalseExpandable, HasDeclaredIdents, FlattensOutputs, AttributeHooks, CfgFalseReporterContext};

// InvocationCollector is not moved here yet, so its dependencies are commented out for now.
// It will be moved in a later step.
// use crate::InvocationCollector;
// use crate::StripUnconfigured; // Will be moved later
// use crate::errors::RemoveNodeNotSupported; // Will be moved later

/// A trait implemented for various AST nodes, providing all pieces
/// of functionality used by `InvocationCollector`.
///
/// This trait acts as an adapter, allowing the `InvocationCollector` to interact with
/// different AST node types in a generic way. Each implementor defines how it should be
/// processed during macro invocation collection and expansion.
///
/// For n00bs: This is like a "how-to guide" for each type of code snippet (like a function,
/// or a variable declaration). It tells the macro system:
/// - How to get its unique ID (`HasNodeId`).
/// - How to check for attributes (`HasAttrs`).
/// - What kind of output it expects after a macro is expanded (`VisitOutputTy`, `FlatMapOutputTy`).
/// - How to convert itself into a generic `Annotatable` type.
/// - How to handle attributes (`pre_flat_map_node_collect_attr`, `post_flat_map_node_collect_bang`).
/// - How to "walk" its internal structure to find more macro calls (`walk`, `walk_flat_map`).
pub trait InvocationCollectorNode:
    HasAttrs
    + HasNodeId
    + Sized
    + MacroInvocationNode<ItemKind = ast::ItemKind>
    + AstFragmentConverter
    + WalkableAstNode<FlatMapOutputTy = <Self as AstFragmentConverter>::FlatMapOutputTy>
    + CfgFalseExpandable
    + HasDeclaredIdents
    + FlattensOutputs<FlatMapOutputTy = <Self as AstFragmentConverter>::FlatMapOutputTy>
    + AttributeHooks
{
    /// Provides a human-readable description of the node type.
    fn descr() -> &'static str;
}

// TODO: Move `impl InvocationCollectorNode for ...` blocks here after moving
// their dependencies (like `AstNodeWrapper`, `TraitItemTag`, etc.).
// These will eventually be moved from `rustc_expand/src/ast_fragments_split/ast_fragments_node_impls.rs`.