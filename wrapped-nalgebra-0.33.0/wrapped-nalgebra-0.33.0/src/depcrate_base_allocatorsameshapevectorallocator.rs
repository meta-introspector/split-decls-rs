// Generated macro for SameShapeVectorAllocator (trait)
macro_rules! Depcrate_base_allocatorSameShapeVectorAllocator {
() => {
// Module: crate::base::allocator
// Provides: {"SameShapeVectorAllocator"}
// Dependencies: {}
# [doc = " Restricts the given number of rows to be equal."] pub trait SameShapeVectorAllocator < R1 , R2 > : Allocator < R1 > + Allocator < SameShapeR < R1 , R2 > > + SameShapeAllocator < R1 , U1 , R2 , U1 > where R1 : Dim , R2 : Dim , ShapeConstraint : SameNumberOfRows < R1 , R2 > , { }
};
}
