// Generated macro for llvm_features_by_flags (function)
macro_rules! Depcrate_llvm_utilllvm_features_by_flags {
() => {
// Module: crate::llvm_util
// Provides: {"llvm_features_by_flags"}
// Dependencies: {}
# [doc = " The target features for compiler flags other than `-Ctarget-features`."] fn llvm_features_by_flags (sess : & Session , features : & mut Vec < String >) { target_features :: retpoline_features_by_flags (sess , features) ; if sess . opts . unstable_opts . fixed_x18 { if sess . target . arch != "aarch64" { sess . dcx () . emit_fatal (errors :: FixedX18InvalidArch { arch : & sess . target . arch }) ; } else { features . push ("+reserve-x18" . into ()) ; } } }
};
}
