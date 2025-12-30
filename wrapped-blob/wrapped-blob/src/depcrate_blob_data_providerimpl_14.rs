// Generated macro for impl_14 (impl)
macro_rules! Depcrate_blob_data_providerimpl_14 {
() => {
// Module: crate::blob_data_provider
// Provides: {"impl_14"}
// Dependencies: {}
# [doc = " ✨ *Enabled with the `alloc` Cargo feature.*"] # [cfg (feature = "alloc")] impl IterableDynamicDataProvider < BufferMarker > for BlobDataProvider { fn iter_ids_for_marker (& self , marker : DataMarkerInfo ,) -> Result < alloc :: collections :: BTreeSet < DataIdentifierCow < '_ > > , DataError > { self . data . get () . iter_ids (marker) } }
};
}
