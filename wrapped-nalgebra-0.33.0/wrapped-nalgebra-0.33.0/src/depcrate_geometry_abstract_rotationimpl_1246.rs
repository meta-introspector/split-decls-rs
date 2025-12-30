// Generated macro for impl_1246 (impl)
macro_rules! Depcrate_geometry_abstract_rotationimpl_1246 {
() => {
// Module: crate::geometry::abstract_rotation
// Provides: {"impl_1246"}
// Dependencies: {}
impl < T : SimdRealField > AbstractRotation < T , 2 > for UnitComplex < T > where T :: Element : SimdRealField , { # [inline] fn identity () -> Self { Self :: identity () } # [inline] fn inverse (& self) -> Self { self . inverse () } # [inline] fn inverse_mut (& mut self) { self . inverse_mut () } # [inline] fn transform_vector (& self , v : & SVector < T , 2 >) -> SVector < T , 2 > { self * v } # [inline] fn transform_point (& self , p : & Point < T , 2 >) -> Point < T , 2 > { self * p } # [inline] fn inverse_transform_vector (& self , v : & SVector < T , 2 >) -> SVector < T , 2 > { self . inverse_transform_vector (v) } # [inline] fn inverse_transform_point (& self , p : & Point < T , 2 >) -> Point < T , 2 > { self . inverse_transform_point (p) } }
};
}
