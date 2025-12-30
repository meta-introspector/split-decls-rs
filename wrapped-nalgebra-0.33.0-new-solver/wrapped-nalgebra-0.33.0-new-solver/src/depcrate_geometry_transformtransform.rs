// Generated macro for Transform (struct)
macro_rules! Depcrate_geometry_transformTransform {
() => {
// Module: crate::geometry::transform
// Provides: {"Transform"}
// Dependencies: {}
# [doc = " A transformation matrix in homogeneous coordinates."] # [doc = ""] # [doc = " It is stored as a matrix with dimensions `(D + 1, D + 1)`, e.g., it stores a 4x4 matrix for a"] # [doc = " 3D transformation."] # [repr (C)] pub struct Transform < T : RealField , C : TCategory , const D : usize > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , { matrix : OMatrix < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , _phantom : PhantomData < C > , }
};
}
