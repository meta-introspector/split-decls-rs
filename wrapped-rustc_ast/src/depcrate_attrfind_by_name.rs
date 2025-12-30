// Generated macro for find_by_name (function)
macro_rules! Depcrate_attrfind_by_name {
() => {
// Module: crate::attr
// Provides: {"find_by_name"}
// Dependencies: {}
pub fn find_by_name < A : AttributeExt > (attrs : & [A] , name : Symbol) -> Option < & A > { filter_by_name (attrs , name) . next () }
};
}
