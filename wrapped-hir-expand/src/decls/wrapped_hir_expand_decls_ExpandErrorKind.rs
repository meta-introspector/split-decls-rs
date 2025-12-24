use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ExpandErrorKind {
    /// Attribute macro expansion is disabled.
    ProcMacroAttrExpansionDisabled,
    MissingProcMacroExpander(Crate),
    /// The macro for this call is disabled.
    MacroDisabled,
    /// The macro definition has errors.
    MacroDefinition,
    Mbe(mbe::ExpandErrorKind),
    RecursionOverflow,
    Other(Box<str>),
    ProcMacroPanic(Box<str>),
}
