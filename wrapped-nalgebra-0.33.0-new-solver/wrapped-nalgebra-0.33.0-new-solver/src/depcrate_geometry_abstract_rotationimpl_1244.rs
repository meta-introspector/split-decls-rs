// Generated macro for impl_1244 (impl)
macro_rules! Depcrate_geometry_abstract_rotationimpl_1244 {
() => {
// Module: crate::geometry::abstract_rotation
// Provides: {"impl_1244"}
// Dependencies: {}
impl < T : SimdRealField , const D : usize > AbstractRotation < T , D > for Rotation < T , D > where T :: Element : SimdRealField , { # [inline] fn identity () -> Self { Self :: identity () } # [inline] fn inverse (& self) -> Self { self . inverse () } # [inline] fn inverse_mut (& mut self) { self . inverse_mut () } # [inline] fn transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self * v } # [inline] fn transform_point (& self , p : & Point < T , D >) -> Point < T , D > { self * p } # [inline] fn inverse_transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > { self . inverse_transform_vector (v) } # [inline] fn inverse_transform_unit_vector (& self , v : & Unit < SVector < T , D > >) -> Unit < SVector < T , D > > { self . inverse_transform_unit_vector (v) } # [inline] fn inverse_transform_point (& self , p : & Point < T , D >) -> Point < T , D > { self . inverse_transform_point (p) } }
};
}
