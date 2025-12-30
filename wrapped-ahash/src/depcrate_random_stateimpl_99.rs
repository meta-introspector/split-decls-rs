// Generated macro for impl_99 (impl)
macro_rules! Depcrate_random_stateimpl_99 {
() => {
// Module: crate::random_state
// Provides: {"impl_99"}
// Dependencies: {}
impl DefaultRandomSource { fn new () -> DefaultRandomSource { DefaultRandomSource { counter : AtomicUsize :: new (& PI_U64X4 as * const _ as usize) , } } # [cfg (all (target_arch = "arm" , target_os = "none"))] const fn default () -> DefaultRandomSource { DefaultRandomSource { counter : AtomicUsize :: new (PI_U64X4 [3] as usize) , } } }
};
}
