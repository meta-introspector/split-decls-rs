// Generated macro for impl_18 (impl)
macro_rules! Depcrate_fs_data_providerimpl_18 {
() => {
// Module: crate::fs_data_provider
// Provides: {"impl_18"}
// Dependencies: {}
impl DynamicDryDataProvider < BufferMarker > for FsDataProvider { fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { Ok (self . dry_load_internal (marker , req) ? . 0) } }
};
}
