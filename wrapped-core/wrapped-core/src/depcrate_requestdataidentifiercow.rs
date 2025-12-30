// Generated macro for DataIdentifierCow (struct)
macro_rules! Depcrate_requestDataIdentifierCow {
() => {
// Module: crate::request
// Provides: {"DataIdentifierCow"}
// Dependencies: {}
# [doc = " A data identifier identifies a particular version of data, such as \"English\"."] # [doc = ""] # [doc = " It is a wrapper around a [`DataLocale`] and a [`DataMarkerAttributes`]."] # [doc = ""] # [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [derive (Debug , PartialEq , Eq , Hash , Clone)] # [non_exhaustive] # [cfg (feature = "alloc")] pub struct DataIdentifierCow < 'a > { # [doc = " Marker-specific request attributes"] pub marker_attributes : Cow < 'a , DataMarkerAttributes > , # [doc = " The CLDR locale"] pub locale : DataLocale , }
};
}
