// Generated macro for impl_29 (impl)
macro_rules! Depcrateimpl_29 {
() => {
// Module: crate
// Provides: {"impl_29"}
// Dependencies: {}
# [cfg (feature = "blob_input")] impl < M : DataMarker > IterableDataProvider < M > for ReexportableBlobDataProvider where BlobDataProvider : AsDeserializingBufferProvider , for < 'a > DeserializingBufferProvider < 'a , BlobDataProvider > : DataProvider < M > , { fn iter_ids (& self) -> Result < std :: collections :: BTreeSet < DataIdentifierCow < '_ > > , DataError > { self . 0 . iter_ids_for_marker (M :: INFO) } }
};
}
