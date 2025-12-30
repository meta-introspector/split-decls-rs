// Generated macro for filter_by_name (function)
macro_rules! Depcrate_attrfilter_by_name {
() => {
// Module: crate::attr
// Provides: {"filter_by_name"}
// Dependencies: {}
pub fn filter_by_name < A : AttributeExt > (attrs : & [A] , name : Symbol) -> impl Iterator < Item = & A > { attrs . iter () . filter (move | attr | attr . has_name (name)) }
};
}
