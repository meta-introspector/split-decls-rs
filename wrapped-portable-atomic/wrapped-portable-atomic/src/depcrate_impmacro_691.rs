// Generated macro for macro_691 (macro)
macro_rules! Depcrate_impmacro_691 {
() => {
// Module: crate::imp
// Provides: {"macro_691"}
// Dependencies: {}
# [cfg (any (portable_atomic_unsafe_assume_single_core , feature = "critical-section" , target_arch = "avr" , target_arch = "msp430" ,))] # [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (portable_atomic_no_atomic_cas))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (not (target_has_atomic = "ptr")))] items ! { pub (crate) use self :: interrupt :: { AtomicI16 , AtomicI8 , AtomicIsize , AtomicPtr , AtomicU16 , AtomicU8 , AtomicUsize , } ; # [cfg (any (not (target_pointer_width = "16") , feature = "fallback"))] pub (crate) use self :: interrupt :: { AtomicI32 , AtomicU32 } ; # [cfg (any (not (any (target_pointer_width = "16" , target_pointer_width = "32")) , feature = "fallback" ,))] pub (crate) use self :: interrupt :: { AtomicI64 , AtomicU64 } ; # [cfg (feature = "fallback")] pub (crate) use self :: interrupt :: { AtomicI128 , AtomicU128 } ; }
};
}
