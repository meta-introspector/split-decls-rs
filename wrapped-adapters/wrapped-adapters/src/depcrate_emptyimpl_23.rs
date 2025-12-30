// Generated macro for impl_23 (impl)
macro_rules! Depcrate_emptyimpl_23 {
() => {
// Module: crate::empty
// Provides: {"impl_23"}
// Dependencies: {}
impl < M > IterableDataProvider < M > for EmptyDataProvider where M : DataMarker , { fn iter_ids (& self) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { Ok (Default :: default ()) } }
};
}
