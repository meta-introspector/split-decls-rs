// Generated macro for alloc_env_var (function)
macro_rules! Depcrate_shims_unix_envalloc_env_var {
() => {
// Module: crate::shims::unix::env
// Provides: {"alloc_env_var"}
// Dependencies: {}
fn alloc_env_var < 'tcx > (ecx : & mut InterpCx < 'tcx , MiriMachine < 'tcx > > , name : & OsStr , value : & OsStr ,) -> InterpResult < 'tcx , Pointer > { let mut name_osstring = name . to_os_string () ; name_osstring . push ("=") ; name_osstring . push (value) ; ecx . alloc_os_str_as_c_str (name_osstring . as_os_str () , MiriMemoryKind :: Machine . into ()) }
};
}
