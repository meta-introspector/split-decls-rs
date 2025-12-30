// Generated macro for macro_3 (macro)
macro_rules! Depcratemacro_3 {
() => {
// Module: crate
// Provides: {"macro_3"}
// Dependencies: {}
# [cfg (portable_atomic_unsafe_assume_single_core)] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (portable_atomic_no_atomic_cas))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (not (target_has_atomic = "ptr")))] # [cfg (not (any (target_arch = "arm" , target_arch = "avr" , target_arch = "msp430" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "xtensa" ,)))] compile_error ! ("`portable_atomic_unsafe_assume_single_core` cfg (`unsafe-assume-single-core` feature) \
     is not supported yet on this target;\n\
     if you need unsafe-assume-single-core support for this target,\n\
     please submit an issue at <https://github.com/taiki-e/portable-atomic>") ;
};
}
