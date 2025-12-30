// Generated macro for mutex_init_offset (function)
macro_rules! Depcrate_shims_unix_syncmutex_init_offset {
() => {
// Module: crate::shims::unix::sync
// Provides: {"mutex_init_offset"}
// Dependencies: {}
# [doc = " To ensure an initialized mutex that was moved somewhere else can be distinguished from"] # [doc = " a statically initialized mutex that is used the first time, we pick some offset within"] # [doc = " `pthread_mutex_t` and use it as an \"initialized\" flag."] fn mutex_init_offset < 'tcx > (ecx : & MiriInterpCx < 'tcx >) -> InterpResult < 'tcx , Size > { let offset = match & * ecx . tcx . sess . target . os { "linux" | "illumos" | "solaris" | "freebsd" | "android" => 0 , "macos" => 4 , os => throw_unsup_format ! ("`pthread_mutex` is not supported on {os}") , } ; let offset = Size :: from_bytes (offset) ; if ! ecx . machine . pthread_mutex_sanity . replace (true) { let check_static_initializer = | name | { let static_initializer = ecx . eval_path (& ["libc" , name]) ; let init_field = static_initializer . offset (offset , ecx . machine . layouts . u32 , ecx) . unwrap () ; let init = ecx . read_scalar (& init_field) . unwrap () . to_u32 () . unwrap () ; assert_ne ! (init , LAZY_INIT_COOKIE , "{name} is incompatible with our initialization cookie") ; } ; check_static_initializer ("PTHREAD_MUTEX_INITIALIZER") ; match & * ecx . tcx . sess . target . os { "linux" => { check_static_initializer ("PTHREAD_RECURSIVE_MUTEX_INITIALIZER_NP") ; check_static_initializer ("PTHREAD_ERRORCHECK_MUTEX_INITIALIZER_NP") ; check_static_initializer ("PTHREAD_ADAPTIVE_MUTEX_INITIALIZER_NP") ; } "illumos" | "solaris" | "macos" | "freebsd" | "android" => { } os => throw_unsup_format ! ("`pthread_mutex` is not supported on {os}") , } } interp_ok (offset) }
};
}
