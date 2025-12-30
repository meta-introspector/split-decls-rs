// Generated macro for mutexattr_kind_offset (function)
macro_rules! Depcrate_shims_unix_syncmutexattr_kind_offset {
() => {
// Module: crate::shims::unix::sync
// Provides: {"mutexattr_kind_offset"}
// Dependencies: {}
# [inline] fn mutexattr_kind_offset < 'tcx > (ecx : & MiriInterpCx < 'tcx >) -> InterpResult < 'tcx , u64 > { interp_ok (match & * ecx . tcx . sess . target . os { "linux" | "illumos" | "solaris" | "macos" | "freebsd" | "android" => 0 , os => throw_unsup_format ! ("`pthread_mutexattr` is not supported on {os}") , }) }
};
}
