// Generated macro for impl_3282 (impl)
macro_rules! Depcrate_third_party_alga_alga_transformimpl_3282 {
() => {
// Module: crate::third_party::alga::alga_transform
// Provides: {"impl_3282"}
// Dependencies: {}
impl < T , C , const D : usize > ProjectiveTransformation < Point < T , D > > for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , T : RealField + simba :: scalar :: RealField , C : SubTCategoryOf < TProjective > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > + Allocator < DimNameSum < Const < D > , U1 > > , { # [inline] fn inverse_transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . inverse_transform_point (pt) } # [inline] fn inverse_transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self . inverse_transform_vector (v) } }
};
}
