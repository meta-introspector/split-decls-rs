// Generated macro for impl_1517 (impl)
macro_rules! Depcrate_geometry_quaternionimpl_1517 {
() => {
// Module: crate::geometry::quaternion
// Provides: {"impl_1517"}
// Dependencies: {}
impl < T : Scalar + Hash > Hash for Quaternion < T > { fn hash < H : Hasher > (& self , state : & mut H) { self . coords . hash (state) } }
};
}
