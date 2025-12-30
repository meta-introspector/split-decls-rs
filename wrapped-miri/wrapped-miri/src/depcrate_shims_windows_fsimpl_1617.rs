// Generated macro for impl_1617 (impl)
macro_rules! Depcrate_shims_windows_fsimpl_1617 {
() => {
// Module: crate::shims::windows::fs
// Provides: {"impl_1617"}
// Dependencies: {}
impl CreationDisposition { fn new < 'tcx > (value : u32 , ecx : & mut MiriInterpCx < 'tcx > ,) -> InterpResult < 'tcx , CreationDisposition > { let create_always = ecx . eval_windows_u32 ("c" , "CREATE_ALWAYS") ; let create_new = ecx . eval_windows_u32 ("c" , "CREATE_NEW") ; let open_always = ecx . eval_windows_u32 ("c" , "OPEN_ALWAYS") ; let open_existing = ecx . eval_windows_u32 ("c" , "OPEN_EXISTING") ; let truncate_existing = ecx . eval_windows_u32 ("c" , "TRUNCATE_EXISTING") ; let out = if value == create_always { CreationDisposition :: CreateAlways } else if value == create_new { CreationDisposition :: CreateNew } else if value == open_always { CreationDisposition :: OpenAlways } else if value == open_existing { CreationDisposition :: OpenExisting } else if value == truncate_existing { CreationDisposition :: TruncateExisting } else { throw_unsup_format ! ("CreateFileW: Unsupported creation disposition: {value}") ; } ; interp_ok (out) } }
};
}
