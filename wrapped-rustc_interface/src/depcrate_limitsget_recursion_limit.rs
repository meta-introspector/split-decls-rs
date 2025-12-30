// Generated macro for get_recursion_limit (function)
macro_rules! Depcrate_limitsget_recursion_limit {
() => {
// Module: crate::limits
// Provides: {"get_recursion_limit"}
// Dependencies: {}
pub (crate) fn get_recursion_limit (attrs : & [Attribute]) -> Limit { find_attr ! (attrs , AttributeKind :: RecursionLimit { limit , .. } => * limit) . unwrap_or (Limit :: new (128)) }
};
}
