// Generated macro for impl_2520 (impl)
macro_rules! Depcrate_geometry_transformimpl_2520 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2520"}
// Dependencies: {}
impl < T : RealField , C : TCategory , const D : usize > Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , C : SubTCategoryOf < TProjective > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > + Allocator < DimNameSum < Const < D > , U1 > > , { # [doc = " Transform the given point by the inverse of this transformation."] # [doc = " This may be cheaper than inverting the transformation and transforming"] # [doc = " the point."] # [inline] # [must_use] pub fn inverse_transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self . clone () . inverse () * pt } # [doc = " Transform the given vector by the inverse of this transformation."] # [doc = " This may be cheaper than inverting the transformation and transforming"] # [doc = " the vector."] # [inline] # [must_use] pub fn inverse_transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self . clone () . inverse () * v } }
};
}
