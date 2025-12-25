use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Represents different kinds of procedural macros that can be expanded by the external server.
#[derive(Copy, Clone, Eq, PartialEq, Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub enum ProcMacroKind {
    /// A macro that derives implementations for a struct or enum.
    CustomDerive,
    /// An attribute-like procedural macro.
    Attr,
    #[serde(alias = "Bang")]
    #[serde(rename(serialize = "FuncLike", deserialize = "FuncLike"))]
    Bang,
}
