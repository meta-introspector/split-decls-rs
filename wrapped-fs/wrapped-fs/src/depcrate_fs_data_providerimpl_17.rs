// Generated macro for impl_17 (impl)
macro_rules! Depcrate_fs_data_providerimpl_17 {
() => {
// Module: crate::fs_data_provider
// Provides: {"impl_17"}
// Dependencies: {}
impl DynamicDataProvider < BufferMarker > for FsDataProvider { fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < BufferMarker > , DataError > { let (metadata , path) = self . dry_load_internal (marker , req) ? ; let buffer = fs :: read (& path) . map_err (| e | DataError :: from (e) . with_path_context (& path)) ? ; Ok (DataResponse { metadata , payload : DataPayload :: from_owned_buffer (buffer . into_boxed_slice ()) , }) } }
};
}
