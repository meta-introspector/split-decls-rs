// Generated macro for MatrixValueTree (struct)
macro_rules! Depcrate_proptestMatrixValueTree {
() => {
// Module: crate::proptest
// Provides: {"MatrixValueTree"}
// Dependencies: {}
# [doc = " A value tree for matrices."] pub struct MatrixValueTree < T , R , C > where T : Scalar , R : Dim , C : Dim , DefaultAllocator : Allocator < R , C > , { value_tree : Box < dyn ValueTree < Value = OMatrix < T , R , C > > > , }
};
}
