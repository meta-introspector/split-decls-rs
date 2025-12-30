// Generated macro for impl_3276 (impl)
macro_rules! Depcrate_third_party_alga_alga_transformimpl_3276 {
() => {
// Module: crate::third_party::alga::alga_transform
// Provides: {"impl_3276"}
// Dependencies: {}
impl < T : RealField + simba :: scalar :: RealField , C , const D : usize > AbstractMagma < Multiplicative > for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , C : TCategory , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { # [inline] fn operate (& self , rhs : & Self) -> Self { self * rhs } }
};
}
