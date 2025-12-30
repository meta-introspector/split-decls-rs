// Generated macro for macro_8 (macro)
macro_rules! Depcratemacro_8 {
() => {
// Module: crate
// Provides: {"macro_8"}
// Dependencies: {}
# [cfg (portable_atomic_force_amo)] # [cfg (not (any (target_arch = "riscv32" , target_arch = "riscv64")))] compile_error ! ("`portable_atomic_force_amo` cfg (`force-amo` feature) is only available on RISC-V") ;
};
}
