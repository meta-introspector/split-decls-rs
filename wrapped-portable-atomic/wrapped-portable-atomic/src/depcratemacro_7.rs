// Generated macro for macro_7 (macro)
macro_rules! Depcratemacro_7 {
() => {
// Module: crate
// Provides: {"macro_7"}
// Dependencies: {}
# [cfg (portable_atomic_s_mode)] # [cfg (not (any (target_arch = "riscv32" , target_arch = "riscv64")))] compile_error ! ("`portable_atomic_s_mode` cfg (`s-mode` feature) is only available on RISC-V") ;
};
}
