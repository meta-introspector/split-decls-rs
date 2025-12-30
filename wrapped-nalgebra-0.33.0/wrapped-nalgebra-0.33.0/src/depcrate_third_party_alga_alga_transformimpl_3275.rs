// Generated macro for impl_3275 (impl)
macro_rules! Depcrate_third_party_alga_alga_transformimpl_3275 {
() => {
// Module: crate::third_party::alga::alga_transform
// Provides: {"impl_3275"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , C , const D : usize > TwoSidedInverse < Multiplicative > for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , C : SubTCategoryOf < TProjective > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] # [must_use = "Did you mean to use two_sided_inverse_mut()?"] fn two_sided_inverse (& self) -> Self { self . clone () . inverse () } # [inline] fn two_sided_inverse_mut (& mut self) { self . inverse_mut () } }
};
}
