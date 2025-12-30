// Generated macro for macro_2 (macro)
macro_rules! Depcrate_basemacro_2 {
() => {
// Module: crate::base
// Provides: {"macro_2"}
// Dependencies: {}
# [cfg (not (any (target_arch = "arm" , target_arch = "aarch64" , target_arch = "riscv64" , target_arch = "x86" , target_arch = "x86_64")))] compile_error ! ("The target architecture is not supported.") ;
};
}
