// Generated macro for impl_1423 (impl)
macro_rules! Depcrate_shims_windows_fsimpl_1423 {
() => {
// Module: crate::shims::windows::fs
// Provides: {"impl_1423"}
// Dependencies: {}
impl FileDescription for DirHandle { fn name (& self) -> & 'static str { "directory" } fn metadata < 'tcx > (& self) -> InterpResult < 'tcx , io :: Result < Metadata > > { interp_ok (self . path . metadata ()) } fn close < 'tcx > (self , _communicate_allowed : bool , _ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , io :: Result < () > > { interp_ok (Ok (())) } }
};
}
