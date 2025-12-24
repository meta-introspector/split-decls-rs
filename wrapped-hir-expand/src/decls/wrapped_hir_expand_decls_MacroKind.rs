use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MacroKind {
    /// `macro_rules!` or Macros 2.0 macro.
    Declarative,
    /// A built-in function-like macro.
    DeclarativeBuiltIn,
    /// A custom derive.
    Derive,
    /// A builtin-in derive.
    DeriveBuiltIn,
    /// A procedural attribute macro.
    Attr,
    /// A built-in attribute macro.
    AttrBuiltIn,
    /// A function-like procedural macro.
    ProcMacro,
}
