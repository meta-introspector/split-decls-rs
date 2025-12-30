// Generated macro for macro_53 (macro)
macro_rules! Depcrate_simd_funcsmacro_53 {
() => {
// Module: crate::simd_funcs
// Provides: {"macro_53"}
// Dependencies: {}
cfg_if ! { if # [cfg (target_arch = "aarch64")] { macro_rules ! aarch64_return_false_if_below_hebrew { ($ s : ident) => ({ unsafe { if vmaxvq_u16 ($ s . into ()) < 0x0590 { return false ; } } }) } macro_rules ! non_aarch64_return_false_if_all { ($ s : ident) => () } } else { macro_rules ! aarch64_return_false_if_below_hebrew { ($ s : ident) => () } macro_rules ! non_aarch64_return_false_if_all { ($ s : ident) => ({ if all_mask16x8 ($ s) { return false ; } }) } } }
};
}
