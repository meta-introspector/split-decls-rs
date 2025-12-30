// Generated macro for impl_2060 (impl)
macro_rules! Depcrate_geometry_translation_conversionimpl_2060 {
() => {
// Module: crate::geometry::translation_conversion
// Provides: {"impl_2060"}
// Dependencies: {}
impl < T : Scalar + Zero + One , const D : usize > From < Translation < T , D > > for OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > + Allocator < Const < D > > , { # [inline] fn from (t : Translation < T , D >) -> Self { t . to_homogeneous () } }
};
}
