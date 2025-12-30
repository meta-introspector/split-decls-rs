// Generated macro for Convertible (trait)
macro_rules! Depcrate_units_convertibleConvertible {
() => {
// Module: crate::units::convertible
// Provides: {"Convertible"}
// Dependencies: {}
# [doc = " A trait for types that can be converted between two units."] pub trait Convertible : Clone { # [doc = " Adds two values by reference, avoiding data cloning."] fn add_refs (& self , other : & Self) -> Self ; # [doc = " Multiplies two values by reference, avoiding data cloning."] fn mul_refs (& self , other : & Self) -> Self ; # [doc = " Converts a [`Ratio<BigInt>`] to the implementing type."] fn from_ratio_bigint (ratio : Ratio < BigInt >) -> Option < Self > ; # [doc = " Returns the reciprocal of the implementing type."] fn reciprocal (& self) -> Self ; }
};
}
