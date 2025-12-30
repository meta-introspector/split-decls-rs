// Generated macro for OMatrix (type)
macro_rules! Depcrate_base_aliasOMatrix {
() => {
// Module: crate::base::alias
// Provides: {"OMatrix"}
// Dependencies: {}
# [doc = " An owned matrix column-major matrix with `R` rows and `C` columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type OMatrix < T , R , C > = Matrix < T , R , C , Owned < T , R , C > > ;
};
}
