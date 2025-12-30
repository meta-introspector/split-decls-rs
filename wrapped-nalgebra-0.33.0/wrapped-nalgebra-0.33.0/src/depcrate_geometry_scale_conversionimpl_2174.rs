// Generated macro for impl_2174 (impl)
macro_rules! Depcrate_geometry_scale_conversionimpl_2174 {
() => {
// Module: crate::geometry::scale_conversion
// Provides: {"impl_2174"}
// Dependencies: {}
impl < T : Scalar + Zero + One , const D : usize > From < Scale < T , D > > for OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > + Allocator < DimNameSum < Const < D > , U1 > , U1 > + Allocator < Const < D > > , { # [inline] fn from (t : Scale < T , D >) -> Self { t . to_homogeneous () } }
};
}
