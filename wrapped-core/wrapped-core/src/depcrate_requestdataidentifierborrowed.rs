// Generated macro for DataIdentifierBorrowed (struct)
macro_rules! Depcrate_requestDataIdentifierBorrowed {
() => {
// Module: crate::request
// Provides: {"DataIdentifierBorrowed"}
// Dependencies: {}
# [doc = " The borrowed version of a [`DataIdentifierCow`]."] # [derive (Default , Debug , Clone , Copy , PartialEq , Eq)] # [non_exhaustive] pub struct DataIdentifierBorrowed < 'a > { # [doc = " Marker-specific request attributes"] pub marker_attributes : & 'a DataMarkerAttributes , # [doc = " The CLDR locale"] pub locale : & 'a DataLocale , }
};
}
