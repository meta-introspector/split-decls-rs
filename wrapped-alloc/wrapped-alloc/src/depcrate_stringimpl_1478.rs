// Generated macro for impl_1478 (impl)
macro_rules! Depcrate_stringimpl_1478 {
() => {
// Module: crate::string
// Provides: {"impl_1478"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [cfg (feature = "optimize_for_size")] impl SpecToString for u8 { # [inline] fn spec_to_string (& self) -> String { let mut buf = String :: with_capacity (3) ; let mut n = * self ; if n >= 10 { if n >= 100 { buf . push ((b'0' + n / 100) as char) ; n %= 100 ; } buf . push ((b'0' + n / 10) as char) ; n %= 10 ; } buf . push ((b'0' + n) as char) ; buf } }
};
}
