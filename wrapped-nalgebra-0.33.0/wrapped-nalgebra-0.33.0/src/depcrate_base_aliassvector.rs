// Generated macro for SVector (type)
macro_rules! Depcrate_base_aliasSVector {
() => {
// Module: crate::base::alias
// Provides: {"SVector"}
// Dependencies: {}
# [doc = " A statically sized D-dimensional column vector."] pub type SVector < T , const D : usize > = Matrix < T , Const < D > , U1 , ArrayStorage < T , D , 1 > > ;
};
}
