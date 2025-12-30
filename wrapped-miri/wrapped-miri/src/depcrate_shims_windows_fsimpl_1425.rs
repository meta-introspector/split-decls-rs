// Generated macro for impl_1425 (impl)
macro_rules! Depcrate_shims_windows_fsimpl_1425 {
() => {
// Module: crate::shims::windows::fs
// Provides: {"impl_1425"}
// Dependencies: {}
impl FileDescription for MetadataHandle { fn name (& self) -> & 'static str { "metadata-only" } fn metadata < 'tcx > (& self) -> InterpResult < 'tcx , io :: Result < Metadata > > { interp_ok (Ok (self . meta . clone ())) } fn close < 'tcx > (self , _communicate_allowed : bool , _ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , io :: Result < () > > { interp_ok (Ok (())) } }
};
}
