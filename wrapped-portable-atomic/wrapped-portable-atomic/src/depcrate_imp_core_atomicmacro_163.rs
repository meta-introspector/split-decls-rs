// Generated macro for macro_163 (macro)
macro_rules! Depcrate_imp_core_atomicmacro_163 {
() => {
// Module: crate::imp::core_atomic
// Provides: {"macro_163"}
// Dependencies: {}
# [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (not (portable_atomic_no_atomic_64)))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (any (target_has_atomic = "64" , not (any (target_pointer_width = "16" , target_pointer_width = "32")) ,)))] atomic_int ! (AtomicU64 , u64) ;
};
}
