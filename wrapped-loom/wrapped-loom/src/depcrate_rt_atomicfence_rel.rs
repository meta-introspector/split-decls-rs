// Generated macro for fence_rel (function)
macro_rules! Depcrate_rt_atomicfence_rel {
() => {
// Module: crate::rt::atomic
// Provides: {"fence_rel"}
// Dependencies: {}
fn fence_rel (execution : & mut Execution) { let active = execution . threads . active_mut () ; active . released = active . causality ; }
};
}
