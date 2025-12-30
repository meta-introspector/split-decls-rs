// Generated macro for impl_2641 (impl)
macro_rules! Depcrate_geometry_orthographicimpl_2641 {
() => {
// Module: crate::geometry::orthographic
// Provides: {"impl_2641"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : RealField + Serialize > Serialize for Orthographic3 < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . matrix . serialize (serializer) } }
};
}
