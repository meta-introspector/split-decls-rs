// Generated macro for filter_attrs (function)
macro_rules! Depcrate_internals_attributes_fieldfilter_attrs {
() => {
// Module: crate::internals::attributes::field
// Provides: {"filter_attrs"}
// Dependencies: {}
# [cfg (feature = "schema")] pub (crate) fn filter_attrs (attrs : impl Iterator < Item = Attribute > ,) -> impl Iterator < Item = Attribute > { attrs . filter (| attr | attr . path () == BORSH) }
};
}
