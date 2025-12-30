// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
# [cfg (feature = "blob_input")] impl < M : DataMarker > DataProvider < M > for ReexportableBlobDataProvider where BlobDataProvider : AsDeserializingBufferProvider , for < 'a > DeserializingBufferProvider < 'a , BlobDataProvider > : DataProvider < M > , { fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { self . 0 . as_deserializing () . load (req) } }
};
}
