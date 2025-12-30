// Generated macro for has_outer (function)
macro_rules! Depcrate_attrhas_outer {
() => {
// Module: crate::attr
// Provides: {"has_outer"}
// Dependencies: {}
pub fn has_outer (attrs : & [Attribute]) -> bool { for attr in attrs { if let AttrStyle :: Outer = attr . style { return true ; } } false }
};
}
