// Generated macro for impl_972 (impl)
macro_rules! Depcrate_region_infer_valuesimpl_972 {
() => {
// Module: crate::region_infer::values
// Provides: {"impl_972"}
// Dependencies: {}
impl PlaceholderIndices { # [doc = " Returns the `PlaceholderIndex` for the inserted `PlaceholderRegion`"] pub (crate) fn insert (& mut self , placeholder : ty :: PlaceholderRegion) -> PlaceholderIndex { let (index , _) = self . indices . insert_full (placeholder) ; index . into () } pub (crate) fn lookup_index (& self , placeholder : ty :: PlaceholderRegion) -> PlaceholderIndex { self . indices . get_index_of (& placeholder) . unwrap () . into () } pub (crate) fn lookup_placeholder (& self , placeholder : PlaceholderIndex ,) -> ty :: PlaceholderRegion { self . indices [placeholder . index ()] } pub (crate) fn len (& self) -> usize { self . indices . len () } }
};
}
