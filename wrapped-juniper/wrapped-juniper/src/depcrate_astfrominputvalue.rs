// Generated macro for FromInputValue (trait)
macro_rules! Depcrate_astFromInputValue {
() => {
// Module: crate::ast
// Provides: {"FromInputValue"}
// Dependencies: {}
# [doc = " Parsing of an unstructured [`InputValue`] into a Rust data type."] # [doc = ""] # [doc = " The conversion _can_ fail, and must in that case return an [`Err`]. Thus, not restricted in the"] # [doc = " definition of this trait, the returned [`Err`] should be convertible with the [`IntoFieldError`]"] # [doc = " trait to fit well into the library machinery."] # [doc = ""] # [doc = " [`IntoFieldError`]: crate::IntoFieldError"] pub trait FromInputValue < S = DefaultScalarValue > : Sized { # [doc = " Type of this conversion error."] # [doc = ""] # [doc = " Thus, not restricted, it should be convertible with the [`IntoFieldError`] trait to fit well"] # [doc = " into the library machinery."] # [doc = ""] # [doc = " [`IntoFieldError`]: crate::IntoFieldError"] type Error ; # [doc = " Performs the conversion."] fn from_input_value (v : & InputValue < S >) -> Result < Self , Self :: Error > ; # [doc = " Performs the conversion from an absent value (e.g. to distinguish"] # [doc = " between implicit and explicit `null`)."] # [doc = ""] # [doc = " The default implementation just calls [`from_input_value()`] as if an"] # [doc = " explicit `null` was provided."] # [doc = ""] # [doc = " [`from_input_value()`]: FromInputValue::from_input_value"] fn from_implicit_null () -> Result < Self , Self :: Error > { Self :: from_input_value (& InputValue :: < S > :: Null) } }
};
}
