// Generated macro for AffineCoordinates (trait)
macro_rules! Depcrate_pointAffineCoordinates {
() => {
// Module: crate::point
// Provides: {"AffineCoordinates"}
// Dependencies: {}
# [doc = " Access to the affine coordinates of an elliptic curve point."] pub trait AffineCoordinates : Sized { # [doc = " Field element representation with curve-specific serialization/endianness."] type FieldRepr : AsRef < [u8] > ; # [doc = " Creates an affine point from its coordinates."] fn from_coordinates (x : & Self :: FieldRepr , y : & Self :: FieldRepr) -> CtOption < Self > ; # [doc = " Get the affine x-coordinate as a serialized field element."] fn x (& self) -> Self :: FieldRepr ; # [doc = " Get the affine y-coordinate as a serialized field element."] fn y (& self) -> Self :: FieldRepr ; # [doc = " Is the affine x-coordinate odd?"] fn x_is_odd (& self) -> Choice ; # [doc = " Is the affine y-coordinate odd?"] fn y_is_odd (& self) -> Choice ; }
};
}
