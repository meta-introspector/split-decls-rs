// Generated macro for interrupt (module)
macro_rules! Depcrate_impinterrupt {
() => {
// Module: crate::imp
// Provides: {"interrupt"}
// Dependencies: {}
# [cfg (any (all (test , target_os = "none") , portable_atomic_unsafe_assume_single_core , feature = "critical-section" , target_arch = "avr" , target_arch = "msp430" ,))] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (any (test , portable_atomic_no_atomic_cas)))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (any (test , not (target_has_atomic = "ptr"))))] # [cfg (any (target_arch = "arm" , target_arch = "avr" , target_arch = "msp430" , target_arch = "riscv32" , target_arch = "riscv64" , target_arch = "xtensa" , feature = "critical-section" ,))] mod interrupt ;
};
}
