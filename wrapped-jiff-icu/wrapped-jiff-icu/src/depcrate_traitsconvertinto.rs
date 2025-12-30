// Generated macro for ConvertInto (trait)
macro_rules! Depcrate_traitsConvertInto {
() => {
// Module: crate::traits
// Provides: {"ConvertInto"}
// Dependencies: {}
# [doc = " Adds infallible conversions between crates that mirrors [`Into`]."] pub trait ConvertInto < T > : Sized { # [doc = " Infallibly converts a value of type `Self` to a value of type `T`."] fn convert_into (self) -> T ; }
};
}
