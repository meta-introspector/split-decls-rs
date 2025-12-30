// Generated macro for impl_1703 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1703 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1703"}
// Dependencies: {}
# [cfg (feature = "serde-serialize-no-std")] impl < 'a , T : SimdRealField > Deserialize < 'a > for DualQuaternion < T > where T : Deserialize < 'a > , { fn deserialize < Des > (deserializer : Des) -> Result < Self , Des :: Error > where Des : Deserializer < 'a > , { type Dq < T > = [T ; 8] ; let dq : Dq < T > = Dq :: < T > :: deserialize (deserializer) ? ; Ok (Self { real : Quaternion :: new (dq [3] . clone () , dq [0] . clone () , dq [1] . clone () , dq [2] . clone ()) , dual : Quaternion :: new (dq [7] . clone () , dq [4] . clone () , dq [5] . clone () , dq [6] . clone ()) , }) } }
};
}
