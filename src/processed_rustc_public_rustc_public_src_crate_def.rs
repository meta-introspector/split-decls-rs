// SRC: ../rust/compiler/rustc_public/src/crate_def.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=4 | LINES=6 */
// Module that define a common trait for things that represent a crate definition,
// such as, a function, a trait, an enum, and any other definitions.

use serde::Serialize;

use crate::ty::{GenericArgs, Span, Ty};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
use crate::{AssocItems, Crate, Symbol, with};
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=DefId(pub(crate) | COMPLEXITY=11 | LINES=25 */

/// A unique identification number for each item accessible for the current compilation unit.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct DefId(pub(crate) usize);

impl DefId {
    /// Return fully qualified name of this definition
    pub fn name(&self) -> Symbol {
        with(|cx| cx.def_name(*self, false))
    }

    /// Return a trimmed name of this definition.
    ///
    /// This can be used to print more user friendly diagnostic messages.
    ///
    /// If a symbol name can only be imported from one place for a type, and as
    /// long as it was not glob-imported anywhere in the current crate, we trim its
    /// path and print only the name.
    ///
    /// For example, this function may shorten `std::vec::Vec` to just `Vec`,
    /// as long as there is no other `Vec` importable anywhere.
    pub fn trimmed_name(&self) -> Symbol {
        with(|cx| cx.def_name(*self, true))
    }
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=def_id | COMPLEXITY=19 | LINES=55 */

/// A trait for retrieving information about a particular definition.
///
/// Implementors must provide the implementation of `def_id` which will be used to retrieve
/// information about a crate's definition.
pub trait CrateDef {
    /// Retrieve the unique identifier for the current definition.
    fn def_id(&self) -> DefId;

    /// Return the fully qualified name of the current definition.
    ///
    /// See [`DefId::name`] for more details
    fn name(&self) -> Symbol {
        self.def_id().name()
    }

    /// Return a trimmed name of this definition.
    ///
    /// See [`DefId::trimmed_name`] for more details
    fn trimmed_name(&self) -> Symbol {
        self.def_id().trimmed_name()
    }

    /// Return information about the crate where this definition is declared.
    ///
    /// This will return the crate number and its name.
    fn krate(&self) -> Crate {
        let def_id = self.def_id();
        with(|cx| cx.krate(def_id))
    }

    /// Return the span of this definition.
    fn span(&self) -> Span {
        let def_id = self.def_id();
        with(|cx| cx.span_of_an_item(def_id))
    }

    /// Return registered tool attributes with the given attribute name.
    ///
    /// FIXME(jdonszelmann): may panic on non-tool attributes. After more attribute work, non-tool
    /// attributes will simply return an empty list.
    ///
    /// Single segmented name like `#[clippy]` is specified as `&["clippy".to_string()]`.
    /// Multi-segmented name like `#[rustfmt::skip]` is specified as `&["rustfmt".to_string(), "skip".to_string()]`.
    fn tool_attrs(&self, attr: &[Symbol]) -> Vec<Attribute> {
        let def_id = self.def_id();
        with(|cx| cx.tool_attrs(def_id, attr))
    }

    /// Return all tool attributes of this definition.
    fn all_tool_attrs(&self) -> Vec<Attribute> {
        let def_id = self.def_id();
        with(|cx| cx.all_tool_attrs(def_id))
    }
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=ty | COMPLEXITY=7 | LINES=17 */

/// A trait that can be used to retrieve a definition's type.
///
/// Note that not every CrateDef has a type `Ty`. They should not implement this trait.
pub trait CrateDefType: CrateDef {
    /// Returns the type of this crate item.
    fn ty(&self) -> Ty {
        with(|cx| cx.def_ty(self.def_id()))
    }

    /// Retrieve the type of this definition by instantiating and normalizing it with `args`.
    ///
    /// This will panic if instantiation fails.
    fn ty_with_args(&self, args: &GenericArgs) -> Ty {
        with(|cx| cx.def_ty_with_args(self.def_id(), args))
    }
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=associated_items | COMPLEXITY=5 | LINES=8 */

/// A trait for retrieving all items from a definition within a crate.
pub trait CrateDefItems: CrateDef {
    /// Retrieve all associated items from a definition.
    fn associated_items(&self) -> AssocItems {
        with(|cx| cx.associated_items(self.def_id()))
    }
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=Attribute | COMPLEXITY=2 | LINES=6 */

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    value: String,
    span: Span,
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=new | COMPLEXITY=6 | LINES=16 */

impl Attribute {
    pub fn new(value: String, span: Span) -> Attribute {
        Attribute { value, span }
    }

    /// Get the span of this attribute.
    pub fn span(&self) -> Span {
        self.span
    }

    /// Get the string representation of this attribute.
    pub fn as_str(&self) -> &str {
        &self.value
    }
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=def_id | COMPLEXITY=12 | LINES=16 */

macro_rules! crate_def {
    ( $(#[$attr:meta])*
      $vis:vis $name:ident $(;)?
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
        $vis struct $name(pub DefId);

        impl CrateDef for $name {
            fn def_id(&self) -> DefId {
                self.0
            }
        }
    };
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=def_id | COMPLEXITY=15 | LINES=18 */

macro_rules! crate_def_with_ty {
    ( $(#[$attr:meta])*
      $vis:vis $name:ident $(;)?
    ) => {
        $(#[$attr])*
        #[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
        $vis struct $name(pub DefId);

        impl CrateDef for $name {
            fn def_id(&self) -> DefId {
                self.0
            }
        }

        impl CrateDefType for $name {}
    };
}
/* AST_META: AST_ID=11 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=11 | LINES=6 */

macro_rules! impl_crate_def_items {
    ( $name:ident $(;)? ) => {
        impl CrateDefItems for $name {}
    };
}