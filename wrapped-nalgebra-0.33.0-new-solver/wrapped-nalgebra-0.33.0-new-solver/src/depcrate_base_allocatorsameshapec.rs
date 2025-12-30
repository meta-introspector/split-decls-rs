// Generated macro for SameShapeC (type)
macro_rules! Depcrate_base_allocatorSameShapeC {
() => {
// Module: crate::base::allocator
// Provides: {"SameShapeC"}
// Dependencies: {}
# [doc = " The number of columns of the result of a componentwise operation on two matrices."] pub type SameShapeC < C1 , C2 > = < ShapeConstraint as SameNumberOfColumns < C1 , C2 > > :: Representative ;
};
}
