// Generated macro for impl_1702 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1702 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1702"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < T : SimdRealField > Serialize for DualQuaternion < T > where T : Serialize , { fn serialize < S > (& self , serializer : S) -> Result < < S as Serializer > :: Ok , < S as Serializer > :: Error > where S : Serializer , { self . as_ref () . serialize (serializer) } }
};
}
