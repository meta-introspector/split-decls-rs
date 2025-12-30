// Generated macro for impl_25 (impl)
macro_rules! Depcrate_base_allocatorimpl_25 {
() => {
// Module: crate::base::allocator
// Provides: {"impl_25"}
// Dependencies: {}
impl < R1 , R2 , C1 , C2 > SameShapeAllocator < R1 , C1 , R2 , C2 > for DefaultAllocator where R1 : Dim , R2 : Dim , C1 : Dim , C2 : Dim , DefaultAllocator : Allocator < R1 , C1 > + Allocator < SameShapeR < R1 , R2 > , SameShapeC < C1 , C2 > > , ShapeConstraint : SameNumberOfRows < R1 , R2 > + SameNumberOfColumns < C1 , C2 > , { }
};
}
