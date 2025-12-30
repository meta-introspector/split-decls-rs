// Generated macro for core_atomic (module)
macro_rules! Depcrate_impcore_atomic {
() => {
// Module: crate::imp
// Provides: {"core_atomic"}
// Dependencies: {}
# [cfg (not (any (all (portable_atomic_no_atomic_load_store , not (all (target_arch = "bpf" , not (feature = "critical-section"))) ,) , target_arch = "avr" , target_arch = "msp430" ,)))] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (not (all (any (target_arch = "riscv32" , target_arch = "riscv64" , feature = "critical-section" , portable_atomic_unsafe_assume_single_core ,) , portable_atomic_no_atomic_cas ,))))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (not (all (any (target_arch = "riscv32" , target_arch = "riscv64" , feature = "critical-section" , portable_atomic_unsafe_assume_single_core ,) , not (target_has_atomic = "ptr") ,))))] mod core_atomic ;
};
}
