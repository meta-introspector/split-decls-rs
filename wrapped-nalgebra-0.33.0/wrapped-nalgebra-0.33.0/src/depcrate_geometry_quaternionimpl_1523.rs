// Generated macro for impl_1523 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1523 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1523"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : Scalar > Serialize for Quaternion < T > where Owned < T , U4 > : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { self . coords . serialize (serializer) } }
};
}
