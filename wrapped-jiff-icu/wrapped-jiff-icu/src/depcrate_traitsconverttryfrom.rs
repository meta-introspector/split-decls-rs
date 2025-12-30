// Generated macro for ConvertTryFrom (trait)
macro_rules! Depcrate_traitsConvertTryFrom {
() => {
// Module: crate::traits
// Provides: {"ConvertTryFrom"}
// Dependencies: {}
# [doc = " Adds fallible conversions between crates that mirrors [`TryFrom`]."] pub trait ConvertTryFrom < F > : Sized { # [doc = " The type of an error that can occur during a conversion."] # [doc = ""] # [doc = " In this crate, all errors correspond to the [`Error`](crate::Error)"] # [doc = " type."] type Error ; # [doc = " Fallibly converts a value of type `F` to a value of type `Self`."] fn convert_try_from (value : F) -> Result < Self , Self :: Error > ; }
};
}
