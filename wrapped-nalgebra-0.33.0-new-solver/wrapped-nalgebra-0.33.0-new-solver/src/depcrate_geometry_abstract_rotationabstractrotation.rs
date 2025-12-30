// Generated macro for AbstractRotation (trait)
macro_rules! Depcrate_geometry_abstract_rotationAbstractRotation {
() => {
// Module: crate::geometry::abstract_rotation
// Provides: {"AbstractRotation"}
// Dependencies: {}
# [doc = " Trait implemented by rotations that can be used inside of an `Isometry` or `Similarity`."] pub trait AbstractRotation < T : Scalar , const D : usize > : PartialEq + ClosedMulAssign + Clone { # [doc = " The rotation identity."] fn identity () -> Self ; # [doc = " The rotation inverse."] fn inverse (& self) -> Self ; # [doc = " Change `self` to its inverse."] fn inverse_mut (& mut self) ; # [doc = " Apply the rotation to the given vector."] fn transform_vector (& self , v : & SVector < T , D >) -> SVector < T , D > ; # [doc = " Apply the rotation to the given point."] fn transform_point (& self , p : & Point < T , D >) -> Point < T , D > ; # [doc = " Apply the inverse rotation to the given vector."] fn inverse_transform_vector (& self , v : & OVector < T , Const < D > >) -> OVector < T , Const < D > > ; # [doc = " Apply the inverse rotation to the given unit vector."] fn inverse_transform_unit_vector (& self , v : & Unit < SVector < T , D > >) -> Unit < SVector < T , D > > { Unit :: new_unchecked (self . inverse_transform_vector (& * * v)) } # [doc = " Apply the inverse rotation to the given point."] fn inverse_transform_point (& self , p : & Point < T , D >) -> Point < T , D > ; }
};
}
