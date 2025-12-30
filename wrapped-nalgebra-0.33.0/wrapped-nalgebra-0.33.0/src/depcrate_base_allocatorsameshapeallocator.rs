// Generated macro for SameShapeAllocator (trait)
macro_rules! Depcrate_base_allocatorSameShapeAllocator {
() => {
// Module: crate::base::allocator
// Provides: {"SameShapeAllocator"}
// Dependencies: {}
# [doc = " Restricts the given number of rows and columns to be respectively the same."] pub trait SameShapeAllocator < R1 , C1 , R2 , C2 > : Allocator < R1 , C1 > + Allocator < SameShapeR < R1 , R2 > , SameShapeC < C1 , C2 > > where R1 : Dim , R2 : Dim , C1 : Dim , C2 : Dim , ShapeConstraint : SameNumberOfRows < R1 , R2 > + SameNumberOfColumns < C1 , C2 > , { }
};
}
