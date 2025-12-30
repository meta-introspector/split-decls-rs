// Generated macro for impl_54 (impl)
macro_rules! Depcrate_filterimpl_54 {
() => {
// Module: crate::filter
// Provides: {"impl_54"}
// Dependencies: {}
impl < M , D , F > IterableDynamicDataProvider < M > for FilterDataProvider < D , F > where M : DynamicDataMarker , F : Fn (DataIdentifierBorrowed) -> bool , D : IterableDynamicDataProvider < M > , { fn iter_ids_for_marker (& self , marker : DataMarkerInfo ,) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { self . inner . iter_ids_for_marker (marker) . map (| set | { set . into_iter () . filter (| id | (self . predicate) (id . as_borrowed ())) . collect () }) } }
};
}
