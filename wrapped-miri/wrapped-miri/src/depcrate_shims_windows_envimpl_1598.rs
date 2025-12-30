// Generated macro for impl_1598 (impl)
macro_rules! Depcrate_shims_windows_envimpl_1598 {
() => {
// Module: crate::shims::windows::env
// Provides: {"impl_1598"}
// Dependencies: {}
impl WindowsEnvVars { pub (crate) fn new < 'tcx > (_ecx : & mut InterpCx < 'tcx , MiriMachine < 'tcx > > , env_vars : FxHashMap < OsString , OsString > ,) -> InterpResult < 'tcx , Self > { interp_ok (Self { map : env_vars }) } # [doc = " Implementation detail for [`InterpCx::get_env_var`]."] pub (crate) fn get < 'tcx > (& self , name : & OsStr) -> InterpResult < 'tcx , Option < OsString > > { interp_ok (self . map . get (name) . cloned ()) } }
};
}
