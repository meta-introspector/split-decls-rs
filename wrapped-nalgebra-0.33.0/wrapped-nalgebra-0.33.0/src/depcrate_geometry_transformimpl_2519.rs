// Generated macro for impl_2519 (impl)
macro_rules! Depcrate_geometry_transformimpl_2519 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2519"}
// Dependencies: {}
impl < T , C , const D : usize > Transform < T , C , D > where T : RealField , C : TCategory , Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > + Allocator < DimNameSum < Const < D > , U1 > > , { # [doc = " Transform the given point by this transformation."] # [doc = ""] # [doc = " This is the same as the multiplication `self * pt`."] # [inline] # [must_use] pub fn transform_point (& self , pt : & Point < T , D >) -> Point < T , D > { self * pt } # [doc = " Transform the given vector by this transformation, ignoring the"] # [doc = " translational component of the transformation."] # [doc = ""] # [doc = " This is the same as the multiplication `self * v`."] # [inline] # [must_use] pub fn transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self * v } }
};
}
