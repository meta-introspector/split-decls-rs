// Generated macro for impl_55 (impl)
macro_rules! Depcrate_filterimpl_55 {
() => {
// Module: crate::filter
// Provides: {"impl_55"}
// Dependencies: {}
impl < M , D , F > IterableDataProvider < M > for FilterDataProvider < D , F > where M : DataMarker , F : Fn (DataIdentifierBorrowed) -> bool , D : IterableDataProvider < M > , { fn iter_ids (& self) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { self . inner . iter_ids () . map (| vec | { vec . into_iter () . filter (| id | (self . predicate) (id . as_borrowed ())) . collect () }) } }
};
}
