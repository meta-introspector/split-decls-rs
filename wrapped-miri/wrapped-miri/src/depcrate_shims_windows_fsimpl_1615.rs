// Generated macro for impl_1615 (impl)
macro_rules! Depcrate_shims_windows_fsimpl_1615 {
() => {
// Module: crate::shims::windows::fs
// Provides: {"impl_1615"}
// Dependencies: {}
impl FileDescription for MetadataHandle { fn name (& self) -> & 'static str { "metadata-only" } fn metadata < 'tcx > (& self) -> InterpResult < 'tcx , io :: Result < Metadata > > { interp_ok (Ok (self . meta . clone ())) } fn destroy < 'tcx > (self , _self_id : FdId , _communicate_allowed : bool , _ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , io :: Result < () > > { interp_ok (Ok (())) } }
};
}
