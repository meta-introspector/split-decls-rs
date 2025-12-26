use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// The type of a demangled AST node.
/// This is only partial, not all nodes are represented.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DemangleNodeType {
    /// Entering a <prefix> production
    Prefix,
    /// Entering a <template-prefix> production
    TemplatePrefix,
    /// Entering a <template-args> production
    TemplateArgs,
    /// Entering a <unqualified-name> production
    UnqualifiedName,
    /// Entering a <template-param> production
    TemplateParam,
    /// Entering a <decltype> production
    Decltype,
    /// Entering a <data-member-prefix> production
    DataMemberPrefix,
    /// Entering a <nested-name> production
    NestedName,
    /// Entering a <special-name> production that is a vtable.
    VirtualTable,
    /// Additional values may be added in the future. Use a
    /// _ pattern for compatibility.
    __NonExhaustive,
}
