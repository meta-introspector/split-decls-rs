// Generated macro for path_eq_single (function)
macro_rules! Depcrate_commonpath_eq_single {
() => {
// Module: crate::common
// Provides: {"path_eq_single"}
// Dependencies: {}
# [doc = " Checks whether the specified [`syn::Path`] equals to one of specified one-segment"] # [doc = " [`AttrNames::values`]."] pub (crate) fn path_eq_single (path : & syn :: Path , names : impl AttrNames) -> bool { path . segments . len () == 1 && names . values () . iter () . any (| name | path . segments [0] . ident == name) }
};
}
