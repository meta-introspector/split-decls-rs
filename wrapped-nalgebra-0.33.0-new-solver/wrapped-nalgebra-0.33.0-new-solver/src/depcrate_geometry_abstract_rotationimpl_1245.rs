// Generated macro for impl_1245 (impl)
macro_rules! Depcrate_geometry_abstract_rotationimpl_1245 {
() => {
// Module: crate::geometry::abstract_rotation
// Provides: {"impl_1245"}
// Dependencies: {}
impl < T : SimdRealField > AbstractRotation < T , 3 > for UnitQuaternion < T > where T :: Element : SimdRealField , { # [inline] fn identity () -> Self { Self :: identity () } # [inline] fn inverse (& self) -> Self { self . inverse () } # [inline] fn inverse_mut (& mut self) { self . inverse_mut () } # [inline] fn transform_vector (& self , v : & SVector < T , 3 >) -> SVector < T , 3 > { self * v } # [inline] fn transform_point (& self , p : & Point < T , 3 >) -> Point < T , 3 > { self * p } # [inline] fn inverse_transform_vector (& self , v : & SVector < T , 3 >) -> SVector < T , 3 > { self . inverse_transform_vector (v) } # [inline] fn inverse_transform_point (& self , p : & Point < T , 3 >) -> Point < T , 3 > { self . inverse_transform_point (p) } }
};
}
