// Generated macro for impl_1698 (impl)
macro_rules! Depcrate_geometry_dual_quaternionimpl_1698 {
() => {
// Module: crate::geometry::dual_quaternion
// Provides: {"impl_1698"}
// Dependencies: {}
impl < T : Scalar + Zero > Default for DualQuaternion < T > { fn default () -> Self { Self { real : Quaternion :: default () , dual : Quaternion :: default () , } } }
};
}
