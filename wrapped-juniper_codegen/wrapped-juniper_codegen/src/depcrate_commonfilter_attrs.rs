// Generated macro for filter_attrs (function)
macro_rules! Depcrate_commonfilter_attrs {
() => {
// Module: crate::common
// Provides: {"filter_attrs"}
// Dependencies: {}
# [doc = " Filters the provided [`syn::Attribute`] to contain only ones with the"] # [doc = " specified `name`."] pub (crate) fn filter_attrs < 'a > (names : impl AttrNames + 'a , attrs : & 'a [syn :: Attribute] ,) -> impl Iterator < Item = & 'a syn :: Attribute > + 'a { attrs . iter () . filter (move | attr | path_eq_single (attr . path () , names)) }
};
}
