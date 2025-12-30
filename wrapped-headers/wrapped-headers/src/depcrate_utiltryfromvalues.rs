// Generated macro for TryFromValues (trait)
macro_rules! Depcrate_utilTryFromValues {
() => {
// Module: crate::util
// Provides: {"TryFromValues"}
// Dependencies: {}
# [doc = " A helper trait for use when deriving `Header`."] pub (crate) trait TryFromValues : Sized { # [doc = " Try to convert from the values into an instance of `Self`."] fn try_from_values < 'i , I > (values : & mut I) -> Result < Self , Error > where Self : Sized , I : Iterator < Item = & 'i HeaderValue > ; }
};
}
