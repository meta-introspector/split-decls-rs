// Generated macro for contains_name (function)
macro_rules! Depcrate_attrcontains_name {
() => {
// Module: crate::attr
// Provides: {"contains_name"}
// Dependencies: {}
pub fn contains_name (attrs : & [impl AttributeExt] , name : Symbol) -> bool { find_by_name (attrs , name) . is_some () }
};
}
