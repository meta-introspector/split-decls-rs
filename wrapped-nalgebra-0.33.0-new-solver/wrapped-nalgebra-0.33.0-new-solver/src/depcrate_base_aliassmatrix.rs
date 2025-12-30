// Generated macro for SMatrix (type)
macro_rules! Depcrate_base_aliasSMatrix {
() => {
// Module: crate::base::alias
// Provides: {"SMatrix"}
// Dependencies: {}
# [doc = " A statically sized column-major matrix with `R` rows and `C` columns."] # [doc = ""] # [doc = " **Because this is an alias, not all its methods are listed here. See the [`Matrix`](crate::base::Matrix) type too.**"] pub type SMatrix < T , const R : usize , const C : usize > = Matrix < T , Const < R > , Const < C > , ArrayStorage < T , R , C > > ;
};
}
