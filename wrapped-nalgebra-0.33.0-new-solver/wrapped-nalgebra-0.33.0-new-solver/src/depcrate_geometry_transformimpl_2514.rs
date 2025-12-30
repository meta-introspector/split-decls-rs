// Generated macro for impl_2514 (impl)
macro_rules! Depcrate_geometry_transformimpl_2514 {
() => {
// Module: crate::geometry::transform
// Provides: {"impl_2514"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : RealField , C : TCategory , const D : usize > Serialize for Transform < T , C , D > where Const < D > : DimNameAdd < U1 > , DefaultAllocator : Allocator < DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > , Owned < T , DimNameSum < Const < D > , U1 > , DimNameSum < Const < D > , U1 > > : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . matrix . serialize (serializer) } }
};
}
