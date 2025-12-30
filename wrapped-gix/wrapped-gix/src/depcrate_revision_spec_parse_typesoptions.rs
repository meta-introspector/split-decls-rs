// Generated macro for Options (struct)
macro_rules! Depcrate_revision_spec_parse_typesOptions {
() => {
// Module: crate::revision::spec::parse::types
// Provides: {"Options"}
// Dependencies: {}
# [doc = " Options for use in [`revision::Spec::from_bstr()`][crate::revision::Spec::from_bstr()]."] # [derive (Debug , Default , Copy , Clone)] pub struct Options { # [doc = " What to do if both refs and object names match the same input."] pub refs_hint : RefsHint , # [doc = " The hint to use when encountering multiple object matching a prefix."] # [doc = ""] # [doc = " If `None`, the rev-spec itself must disambiguate the object by drilling down to desired kinds or applying"] # [doc = " other disambiguating transformations."] pub object_kind_hint : Option < ObjectKindHint > , }
};
}
