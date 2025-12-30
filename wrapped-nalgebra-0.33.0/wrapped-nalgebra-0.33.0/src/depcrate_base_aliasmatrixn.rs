// Generated macro for MatrixN (type)
macro_rules! Depcrate_base_aliasMatrixN {
() => {
// Module: crate::base::alias
// Provides: {"MatrixN"}
// Dependencies: {}
# [doc = " An owned matrix column-major matrix with `D` columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated (note = "use OMatrix<T, D, D> or SMatrix<T, D, D> instead.")] pub type MatrixN < T , D > = Matrix < T , D , D , Owned < T , D , D > > ;
};
}
