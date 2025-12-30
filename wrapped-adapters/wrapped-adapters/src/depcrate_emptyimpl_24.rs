// Generated macro for impl_24 (impl)
macro_rules! Depcrate_emptyimpl_24 {
() => {
// Module: crate::empty
// Provides: {"impl_24"}
// Dependencies: {}
impl < M > IterableDynamicDataProvider < M > for EmptyDataProvider where M : DynamicDataMarker , { fn iter_ids_for_marker (& self , _ : DataMarkerInfo ,) -> Result < BTreeSet < DataIdentifierCow < '_ > > , DataError > { Ok (Default :: default ()) } }
};
}
