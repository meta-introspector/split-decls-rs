// Generated macro for impl_35 (impl)
macro_rules! Depcrate_fallbackimpl_35 {
() => {
// Module: crate::fallback
// Provides: {"impl_35"}
// Dependencies: {}
impl < P , M > DataProvider < M > for LocaleFallbackProvider < P > where P : DataProvider < M > , M : DataMarker , { fn load (& self , req : DataRequest) -> Result < DataResponse < M > , DataError > { self . run_fallback (M :: INFO , req , | req | self . inner . load (req) , | res | & mut res . metadata ,) } }
};
}
