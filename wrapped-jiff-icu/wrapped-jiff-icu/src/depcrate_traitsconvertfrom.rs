// Generated macro for ConvertFrom (trait)
macro_rules! Depcrate_traitsConvertFrom {
() => {
// Module: crate::traits
// Provides: {"ConvertFrom"}
// Dependencies: {}
# [doc = " Adds infallible conversions between crates that mirrors [`From`]."] pub trait ConvertFrom < F > : Sized { # [doc = " Infallibly converts a value of type `F` to a value of type `Self`."] fn convert_from (value : F) -> Self ; }
};
}
