// Generated macro for has_inner (function)
macro_rules! Depcrate_attrhas_inner {
() => {
// Module: crate::attr
// Provides: {"has_inner"}
// Dependencies: {}
pub fn has_inner (attrs : & [Attribute]) -> bool { for attr in attrs { if let AttrStyle :: Inner (_) = attr . style { return true ; } } false }
};
}
