// Generated macro for impl_138 (impl)
macro_rules! Depcrate_data_providerimpl_138 {
() => {
// Module: crate::data_provider
// Provides: {"impl_138"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < M , P > IterableDynamicDataProvider < M > for alloc :: boxed :: Box < P > where M : DynamicDataMarker , P : IterableDynamicDataProvider < M > + ? Sized , { fn iter_ids_for_marker (& self , marker : DataMarkerInfo ,) -> Result < alloc :: collections :: BTreeSet < DataIdentifierCow < '_ > > , DataError > { (* * self) . iter_ids_for_marker (marker) } }
};
}
