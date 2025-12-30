// Generated macro for impl_1269 (impl)
macro_rules! Depcrate_geometry_pointimpl_1269 {
() => {
// Module: crate::geometry::point
// Provides: {"impl_1269"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T : Scalar , D : DimName > Deserialize < 'a > for OPoint < T , D > where DefaultAllocator : Allocator < D > , < DefaultAllocator as Allocator < D > > :: Buffer < T > : Deserialize < 'a > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { let coords = OVector :: < T , D > :: deserialize (deserializer) ? ; Ok (Self :: from (coords)) } }
};
}
