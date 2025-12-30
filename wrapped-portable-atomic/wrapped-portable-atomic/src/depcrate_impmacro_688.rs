// Generated macro for macro_688 (macro)
macro_rules! Depcrate_impmacro_688 {
() => {
// Module: crate::imp
// Provides: {"macro_688"}
// Dependencies: {}
# [cfg (not (any (portable_atomic_no_atomic_load_store , target_arch = "avr" , target_arch = "msp430" ,)))] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (not (all (any (target_arch = "riscv32" , target_arch = "riscv64" , feature = "critical-section" , portable_atomic_unsafe_assume_single_core ,) , portable_atomic_no_atomic_cas ,))))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (not (all (any (target_arch = "riscv32" , target_arch = "riscv64" , feature = "critical-section" , portable_atomic_unsafe_assume_single_core ,) , not (target_has_atomic = "ptr") ,))))] items ! { pub (crate) use self :: core_atomic :: { AtomicI16 , AtomicI32 , AtomicI8 , AtomicIsize , AtomicPtr , AtomicU16 , AtomicU32 , AtomicU8 , AtomicUsize , } ; # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (any (not (portable_atomic_no_atomic_64) , not (any (target_pointer_width = "16" , target_pointer_width = "32")) ,)))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (any (target_has_atomic = "64" , not (any (target_pointer_width = "16" , target_pointer_width = "32")) ,)))] pub (crate) use self :: core_atomic :: { AtomicI64 , AtomicU64 } ; }
};
}
