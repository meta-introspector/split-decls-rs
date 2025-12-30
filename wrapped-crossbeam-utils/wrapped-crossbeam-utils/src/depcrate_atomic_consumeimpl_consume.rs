// Generated macro for impl_consume (macro)
macro_rules! Depcrate_atomic_consumeimpl_consume {
() => {
// Module: crate::atomic::consume
// Provides: {"impl_consume"}
// Dependencies: {}
# [cfg (not (crossbeam_no_atomic))] # [cfg (not (all (any (target_arch = "arm" , target_arch = "aarch64") , not (any (miri , crossbeam_loom , crossbeam_sanitize_thread)) ,)))] macro_rules ! impl_consume { () => { # [inline] fn load_consume (& self) -> Self :: Val { self . load (Ordering :: Acquire) } } ; }
};
}
