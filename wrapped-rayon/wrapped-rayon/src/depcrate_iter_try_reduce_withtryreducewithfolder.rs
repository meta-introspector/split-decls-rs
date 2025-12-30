// Generated macro for TryReduceWithFolder (struct)
macro_rules! Depcrate_iter_try_reduce_withTryReduceWithFolder {
() => {
// Module: crate::iter::try_reduce_with
// Provides: {"TryReduceWithFolder"}
// Dependencies: {}
struct TryReduceWithFolder < 'r , R , T : Try > { reduce_op : & 'r R , opt_control : Option < ControlFlow < T :: Residual , T :: Output > > , full : & 'r AtomicBool , }
};
}
