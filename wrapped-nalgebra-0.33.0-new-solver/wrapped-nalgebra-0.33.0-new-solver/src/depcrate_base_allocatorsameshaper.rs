// Generated macro for SameShapeR (type)
macro_rules! Depcrate_base_allocatorSameShapeR {
() => {
// Module: crate::base::allocator
// Provides: {"SameShapeR"}
// Dependencies: {}
# [doc = " The number of rows of the result of a componentwise operation on two matrices."] pub type SameShapeR < R1 , R2 > = < ShapeConstraint as SameNumberOfRows < R1 , R2 > > :: Representative ;
};
}
