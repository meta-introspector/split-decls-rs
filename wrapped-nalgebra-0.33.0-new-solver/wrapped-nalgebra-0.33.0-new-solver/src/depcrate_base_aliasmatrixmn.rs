// Generated macro for MatrixMN (type)
macro_rules! Depcrate_base_aliasMatrixMN {
() => {
// Module: crate::base::alias
// Provides: {"MatrixMN"}
// Dependencies: {}
# [doc = " An owned matrix column-major matrix with `R` rows and `C` columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] # [deprecated (note = "use SMatrix for a statically-sized matrix using integer dimensions, or OMatrix for an owned matrix using types as dimensions.")] pub type MatrixMN < T , R , C > = Matrix < T , R , C , Owned < T , R , C > > ;
};
}
