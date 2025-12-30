// Generated macro for to_string_str (macro)
macro_rules! Depcrate_stringto_string_str {
() => {
// Module: crate::string
// Provides: {"to_string_str"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] macro_rules ! to_string_str { { $ ($ type : ty ,) * } => { $ (impl SpecToString for $ type { # [inline] fn spec_to_string (& self) -> String { let s : & str = self ; String :: from (s) } }) * } ; }
};
}
