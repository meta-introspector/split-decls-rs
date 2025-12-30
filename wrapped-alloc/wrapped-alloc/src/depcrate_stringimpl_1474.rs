// Generated macro for impl_1474 (impl)
macro_rules! Depcrate_stringimpl_1474 {
() => {
// Module: crate::string
// Provides: {"impl_1474"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl SpecToString for char { # [inline] fn spec_to_string (& self) -> String { String :: from (self . encode_utf8 (& mut [0 ; char :: MAX_LEN_UTF8])) } }
};
}
