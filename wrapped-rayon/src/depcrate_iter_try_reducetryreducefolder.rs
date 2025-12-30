// Generated macro for TryReduceFolder (struct)
macro_rules! Depcrate_iter_try_reduceTryReduceFolder {
() => {
// Module: crate::iter::try_reduce
// Provides: {"TryReduceFolder"}
// Dependencies: {}
struct TryReduceFolder < 'r , R , T : Try > { reduce_op : & 'r R , control : ControlFlow < T :: Residual , T :: Output > , full : & 'r AtomicBool , }
};
}
