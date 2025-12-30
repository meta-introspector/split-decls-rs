// Generated macro for impl_13 (impl)
macro_rules! Depcrate_blob_data_providerimpl_13 {
() => {
// Module: crate::blob_data_provider
// Provides: {"impl_13"}
// Dependencies: {}
impl DynamicDryDataProvider < BufferMarker > for BlobDataProvider { fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { self . data . get () . load (marker , req) ? ; let mut metadata = DataResponseMetadata :: default () ; metadata . buffer_format = Some (BufferFormat :: Postcard1) ; Ok (metadata) } }
};
}
