// Generated macro for impl_36 (impl)
macro_rules! Depcrate_fallbackimpl_36 {
() => {
// Module: crate::fallback
// Provides: {"impl_36"}
// Dependencies: {}
impl < P , M > DryDataProvider < M > for LocaleFallbackProvider < P > where P : DryDataProvider < M > , M : DataMarker , { fn dry_load (& self , req : DataRequest) -> Result < DataResponseMetadata , DataError > { self . run_fallback (M :: INFO , req , | req | self . inner . dry_load (req) , | m | m) } }
};
}
