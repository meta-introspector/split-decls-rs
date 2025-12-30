// Generated macro for impl_3177 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3177 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3177"}
// Dependencies: {}
impl < T , R : DimName , C : DimName > Identity < Additive > for OMatrix < T , R , C > where T : Scalar + Zero , DefaultAllocator : Allocator < R , C > , { # [inline] fn identity () -> Self { Self :: from_element (T :: zero ()) } }
};
}
