// Generated macro for impl_715 (impl)
macro_rules! Depcrate_base_constructionimpl_715 {
() => {
// Module: crate::base::construction
// Provides: {"impl_715"}
// Dependencies: {}
impl < T , R : DimName , C : DimName > Zero for OMatrix < T , R , C > where T : Scalar + Zero + ClosedAddAssign , DefaultAllocator : Allocator < R , C > , { # [inline] fn zero () -> Self { Self :: from_element (T :: zero ()) } # [inline] fn is_zero (& self) -> bool { self . iter () . all (| e | e . is_zero ()) } }
};
}
