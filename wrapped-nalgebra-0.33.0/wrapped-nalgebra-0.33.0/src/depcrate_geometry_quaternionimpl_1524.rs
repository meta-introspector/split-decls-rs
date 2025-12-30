// Generated macro for impl_1524 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1524 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1524"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T : Scalar > Deserialize < 'a > for Quaternion < T > where Owned < T , U4 > : Deserialize < 'a > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { let coords = Vector4 :: < T > :: deserialize (deserializer) ? ; Ok (Self :: from (coords)) } }
};
}
