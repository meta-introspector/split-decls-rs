// Generated macro for impl_27 (impl)
macro_rules! Depcrate_base_allocatorimpl_27 {
() => {
// Module: crate::base::allocator
// Provides: {"impl_27"}
// Dependencies: {}
impl < R1 , R2 > SameShapeVectorAllocator < R1 , R2 > for DefaultAllocator where R1 : Dim , R2 : Dim , DefaultAllocator : Allocator < R1 , U1 > + Allocator < SameShapeR < R1 , R2 > > , ShapeConstraint : SameNumberOfRows < R1 , R2 > , { }
};
}
