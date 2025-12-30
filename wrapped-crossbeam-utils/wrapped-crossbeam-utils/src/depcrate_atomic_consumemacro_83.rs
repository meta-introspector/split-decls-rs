// Generated macro for macro_83 (macro)
macro_rules! Depcrate_atomic_consumemacro_83 {
() => {
// Module: crate::atomic::consume
// Provides: {"macro_83"}
// Dependencies: {}
# [cfg (any (target_has_atomic = "64" , not (any (target_pointer_width = "16" , target_pointer_width = "32")) ,))] impl_atomic ! (AtomicI64 , i64) ;
};
}
