// Generated macro for impl_2664 (impl)
macro_rules! Depcrate_geometry_perspectiveimpl_2664 {
() => {
// Module: crate::geometry::perspective
// Provides: {"impl_2664"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : RealField + Serialize > Serialize for Perspective3 < T > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . matrix . serialize (serializer) } }
};
}
