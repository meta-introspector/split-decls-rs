// Generated macro for macro_162 (macro)
macro_rules! Depcrate_imp_core_atomicmacro_162 {
() => {
// Module: crate::imp::core_atomic
// Provides: {"macro_162"}
// Dependencies: {}
# [cfg_attr (portable_atomic_no_cfg_target_has_atomic , cfg (not (portable_atomic_no_atomic_64)))] # [cfg_attr (not (portable_atomic_no_cfg_target_has_atomic) , cfg (any (target_has_atomic = "64" , not (any (target_pointer_width = "16" , target_pointer_width = "32")) ,)))] atomic_int ! (AtomicI64 , i64) ;
};
}
