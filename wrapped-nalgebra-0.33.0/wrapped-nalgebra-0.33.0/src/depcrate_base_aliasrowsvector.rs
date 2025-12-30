// Generated macro for RowSVector (type)
macro_rules! Depcrate_base_aliasRowSVector {
() => {
// Module: crate::base::alias
// Provides: {"RowSVector"}
// Dependencies: {}
# [doc = " A statically sized D-dimensional row vector."] pub type RowSVector < T , const D : usize > = Matrix < T , U1 , Const < D > , ArrayStorage < T , 1 , D > > ;
};
}
