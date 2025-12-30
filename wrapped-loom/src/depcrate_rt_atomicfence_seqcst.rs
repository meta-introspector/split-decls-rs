// Generated macro for fence_seqcst (function)
macro_rules! Depcrate_rt_atomicfence_seqcst {
() => {
// Module: crate::rt::atomic
// Provides: {"fence_seqcst"}
// Dependencies: {}
fn fence_seqcst (execution : & mut Execution) { fence_acqrel (execution) ; execution . threads . seq_cst_fence () ; }
};
}
