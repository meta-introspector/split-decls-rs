// Generated macro for macro_4 (macro)
macro_rules! Depcratemacro_4 {
() => {
// Module: crate
// Provides: {"macro_4"}
// Dependencies: {}
# [cfg (portable_atomic_no_outline_atomics)] # [cfg (not (any (target_arch = "aarch64" , target_arch = "arm" , target_arch = "arm64ec" , target_arch = "powerpc64" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "x86_64" ,)))] compile_error ! ("`portable_atomic_no_outline_atomics` cfg does not compatible with this target") ;
};
}
