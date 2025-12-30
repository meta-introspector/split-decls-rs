// Generated macro for impl_3179 (impl)
macro_rules! Depcrate_third_party_alga_alga_matriximpl_3179 {
() => {
// Module: crate::third_party::alga::alga_matrix
// Provides: {"impl_3179"}
// Dependencies: {}
impl < T , R : DimName , C : DimName > TwoSidedInverse < Additive > for OMatrix < T , R , C > where T : Scalar + ClosedNeg , DefaultAllocator : Allocator < R , C > , { # [inline] # [must_use = "Did you mean to use two_sided_inverse_mut()?"] fn two_sided_inverse (& self) -> Self { - self } # [inline] fn two_sided_inverse_mut (& mut self) { * self = - self . clone () } }
};
}
