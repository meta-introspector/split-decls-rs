// Generated macro for fence_acqrel (function)
macro_rules! Depcrate_rt_atomicfence_acqrel {
() => {
// Module: crate::rt::atomic
// Provides: {"fence_acqrel"}
// Dependencies: {}
fn fence_acqrel (execution : & mut Execution) { fence_acq (execution) ; fence_rel (execution) ; }
};
}
