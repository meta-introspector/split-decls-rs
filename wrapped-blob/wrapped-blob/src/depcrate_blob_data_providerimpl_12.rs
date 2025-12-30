// Generated macro for impl_12 (impl)
macro_rules! Depcrate_blob_data_providerimpl_12 {
() => {
// Module: crate::blob_data_provider
// Provides: {"impl_12"}
// Dependencies: {}
impl DynamicDataProvider < BufferMarker > for BlobDataProvider { fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < BufferMarker > , DataError > { let payload : Yoke < (& [u8] , Option < u64 >) , Option < Cart > > = self . data . try_map_project_cloned (| blob , _ | blob . load (marker , req)) ? ; let mut metadata = DataResponseMetadata :: default () ; metadata . buffer_format = Some (BufferFormat :: Postcard1) ; metadata . checksum = payload . get () . 1 ; Ok (DataResponse { metadata , payload : DataPayload :: from_yoked_buffer (payload . map_project (| (bytes , _) , _ | bytes)) , }) } }
};
}
