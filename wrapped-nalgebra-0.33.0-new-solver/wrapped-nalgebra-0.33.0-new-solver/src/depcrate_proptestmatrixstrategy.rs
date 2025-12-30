// Generated macro for MatrixStrategy (struct)
macro_rules! Depcrate_proptestMatrixStrategy {
() => {
// Module: crate::proptest
// Provides: {"MatrixStrategy"}
// Dependencies: {}
# [doc = " A strategy for generating matrices."] # [derive (Debug , Clone)] pub struct MatrixStrategy < NStrategy , R : Dim , C : Dim > where NStrategy : Strategy , NStrategy :: Value : Scalar , DefaultAllocator : Allocator < R , C > , { strategy : BoxedStrategy < OMatrix < NStrategy :: Value , R , C > > , }
};
}
