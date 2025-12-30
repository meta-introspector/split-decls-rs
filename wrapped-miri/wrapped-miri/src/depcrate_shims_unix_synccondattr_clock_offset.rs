// Generated macro for condattr_clock_offset (function)
macro_rules! Depcrate_shims_unix_synccondattr_clock_offset {
() => {
// Module: crate::shims::unix::sync
// Provides: {"condattr_clock_offset"}
// Dependencies: {}
# [inline] fn condattr_clock_offset < 'tcx > (ecx : & MiriInterpCx < 'tcx >) -> InterpResult < 'tcx , u64 > { interp_ok (match & * ecx . tcx . sess . target . os { "linux" | "illumos" | "solaris" | "freebsd" | "android" => 0 , os => throw_unsup_format ! ("`pthread_condattr` clock field is not supported on {os}") , }) }
};
}
