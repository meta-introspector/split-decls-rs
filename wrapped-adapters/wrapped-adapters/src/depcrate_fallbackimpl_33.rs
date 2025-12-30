// Generated macro for impl_33 (impl)
macro_rules! Depcrate_fallbackimpl_33 {
() => {
// Module: crate::fallback
// Provides: {"impl_33"}
// Dependencies: {}
impl < P , M > DynamicDataProvider < M > for LocaleFallbackProvider < P > where P : DynamicDataProvider < M > , M : DynamicDataMarker , { fn load_data (& self , marker : DataMarkerInfo , req : DataRequest ,) -> Result < DataResponse < M > , DataError > { self . run_fallback (marker , req , | req | self . inner . load_data (marker , req) , | res | & mut res . metadata ,) } }
};
}
