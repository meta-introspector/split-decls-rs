// Generated macro for impl_3281 (impl)
macro_rules! Depcrate_third_party_alga_alga_transformimpl_3281 {
() => {
// Module: crate::third_party::alga::alga_transform
// Provides: {"impl_3281"}
// Dependencies: {}
impl < T , C , const D : usize > Transformation < Point < T , D > > for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , T : RealField + simba :: scalar :: RealField , C : TCategory , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > + Allocator < DimNameSum < Const < D > , U1 > > , { # [inline] fn transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . transform_point (pt) } # [inline] fn transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self . transform_vector (v) } }
};
}
