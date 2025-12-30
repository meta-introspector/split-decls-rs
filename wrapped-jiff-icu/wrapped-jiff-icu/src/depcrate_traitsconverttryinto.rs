// Generated macro for ConvertTryInto (trait)
macro_rules! Depcrate_traitsConvertTryInto {
() => {
// Module: crate::traits
// Provides: {"ConvertTryInto"}
// Dependencies: {}
# [doc = " Adds fallible conversions between crates that mirrors [`TryInto`]."] pub trait ConvertTryInto < T > : Sized { # [doc = " The type of an error that can occur during a conversion."] # [doc = ""] # [doc = " In this crate, all errors correspond to the [`Error`](crate::Error)"] # [doc = " type."] type Error ; # [doc = " Fallibly converts a value of type `Self` to a value of type `T`."] fn convert_try_into (self) -> Result < T , Self :: Error > ; }
};
}
