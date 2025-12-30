// Generated macro for impl_34 (impl)
macro_rules! Depcrate_fallbackimpl_34 {
() => {
// Module: crate::fallback
// Provides: {"impl_34"}
// Dependencies: {}
impl < P , M > DynamicDryDataProvider < M > for LocaleFallbackProvider < P > where P : DynamicDryDataProvider < M > , M : DynamicDataMarker , { fn dry_load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponseMetadata , DataError > { self . run_fallback (marker , req , | req | self . inner . dry_load_data (marker , req) , | m | m ,) } }
};
}
