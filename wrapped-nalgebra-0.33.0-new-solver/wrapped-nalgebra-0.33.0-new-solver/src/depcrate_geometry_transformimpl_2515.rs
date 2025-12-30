// Generated macro for impl_2515 (impl)
macro_rules! Depcrate_geometry_transformimpl_2515 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2515"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T : RealField , C : TCategory , const D : usize > Deserialize < 'a > for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , Owned < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > : Deserialize < 'a > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { let matrix = OMatrix :: < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > :: deserialize (deserializer ,) ? ; Ok (Transform :: from_matrix_unchecked (matrix)) } }
};
}
