// Generated macro for macro_80 (macro)
macro_rules! Depcrate_atomic_consumemacro_80 {
() => {
// Module: crate::atomic::consume
// Provides: {"macro_80"}
// Dependencies: {}
# [cfg (any (target_has_atomic = "32" , not (target_pointer_width = "16")))] impl_atomic ! (AtomicU32 , u32) ;
};
}
