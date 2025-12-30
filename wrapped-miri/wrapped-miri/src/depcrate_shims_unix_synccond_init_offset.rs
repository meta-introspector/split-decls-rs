// Generated macro for cond_init_offset (function)
macro_rules! Depcrate_shims_unix_synccond_init_offset {
() => {
// Module: crate::shims::unix::sync
// Provides: {"cond_init_offset"}
// Dependencies: {}
fn cond_init_offset < 'tcx > (ecx : & MiriInterpCx < 'tcx >) -> InterpResult < 'tcx , Size > { let offset = match & * ecx . tcx . sess . target . os { "linux" | "illumos" | "solaris" | "freebsd" | "android" => 0 , "macos" => 4 , os => throw_unsup_format ! ("`pthread_cond` is not supported on {os}") , } ; let offset = Size :: from_bytes (offset) ; if ! ecx . machine . pthread_condvar_sanity . replace (true) { let static_initializer = ecx . eval_path (& ["libc" , "PTHREAD_COND_INITIALIZER"]) ; let init_field = static_initializer . offset (offset , ecx . machine . layouts . u32 , ecx) . unwrap () ; let init = ecx . read_scalar (& init_field) . unwrap () . to_u32 () . unwrap () ; assert_ne ! (init , LAZY_INIT_COOKIE , "PTHREAD_COND_INITIALIZER is incompatible with our initialization cookie") ; } interp_ok (offset) }
};
}
