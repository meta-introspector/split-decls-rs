// Generated macro for is_seq_cst (function)
macro_rules! Depcrate_rt_atomicis_seq_cst {
() => {
// Module: crate::rt::atomic
// Provides: {"is_seq_cst"}
// Dependencies: {}
fn is_seq_cst (order : Ordering) -> bool { order == Ordering :: SeqCst }
};
}
