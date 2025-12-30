// Generated macro for TryFoldFolder (struct)
macro_rules! Depcrate_iter_try_foldTryFoldFolder {
() => {
// Module: crate::iter::try_fold
// Provides: {"TryFoldFolder"}
// Dependencies: {}
struct TryFoldFolder < 'r , C , U : Try , F > { base : C , fold_op : & 'r F , control : ControlFlow < U :: Residual , U :: Output > , }
};
}
