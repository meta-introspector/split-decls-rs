// Generated macro for impl_1268 (impl)
macro_rules! Depcrate_geometry_pointimpl_1268 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1268"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : Scalar , D : DimName > Serialize for OPoint < T , D > where DefaultAllocator : Allocator < D > , < DefaultAllocator as Allocator < D > > :: Buffer < T > : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . coords . serialize (serializer) } }
};
}
