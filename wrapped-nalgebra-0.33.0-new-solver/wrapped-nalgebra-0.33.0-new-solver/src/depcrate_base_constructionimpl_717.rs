// Generated macro for impl_717 (impl)
macro_rules! Depcrate_base_constructionimpl_717 {
() => {
// Module: crate::base::construction
// Provides: {"impl_717"}
// Dependencies: {}
impl < T , R : DimName , C : DimName > Bounded for OMatrix < T , R , C > where T : Scalar + Bounded , DefaultAllocator : Allocator < R , C > , { # [inline] fn max_value () -> Self { Self :: from_element (T :: max_value ()) } # [inline] fn min_value () -> Self { Self :: from_element (T :: min_value ()) } }
};
}
