// Generated macro for riscv (module)
macro_rules! Depcrate_impriscv {
() => {
// Module: crate::imp
// Provides: {"riscv"}
// Dependencies: {}
# [cfg (any (test , not (feature = "critical-section")))] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (any (all (test , not (any (miri , portable_atomic_sanitize_thread))) , portable_atomic_no_atomic_cas ,)))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (any (all (test , not (any (miri , portable_atomic_sanitize_thread))) , not (target_has_atomic = "ptr") ,)))] # [cfg (any (target_arch = "riscv32" , target_arch = "riscv64"))] mod riscv ;
};
}
