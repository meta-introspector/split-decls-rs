// Generated macro for macro_690 (macro)
macro_rules! Depcrate_impmacro_690 {
() => {
// Module: crate::imp
// Provides: {"macro_690"}
// Dependencies: {}
# [cfg (not (any (portable_atomic_unsafe_assume_single_core , feature = "critical-section")))] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (portable_atomic_no_atomic_cas))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (not (target_has_atomic = "ptr")))] # [cfg (any (target_arch = "riscv32" , target_arch = "riscv64"))] items ! { pub (crate) use self :: riscv :: { AtomicI16 , AtomicI32 , AtomicI8 , AtomicIsize , AtomicPtr , AtomicU16 , AtomicU32 , AtomicU8 , AtomicUsize , } ; # [cfg (target_arch = "riscv64")] pub (crate) use self :: riscv :: { AtomicI64 , AtomicU64 } ; }
};
}
