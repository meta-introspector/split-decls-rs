// Generated macro for ToString (trait)
macro_rules! Depcrate_stringToString {
() => {
// Module: crate::string
// Provides: {"ToString"}
// Dependencies: {}
# [doc = " A trait for converting a value to a `String`."] # [doc = ""] # [doc = " This trait is automatically implemented for any type which implements the"] # [doc = " [`Display`] trait. As such, `ToString` shouldn't be implemented directly:"] # [doc = " [`Display`] should be implemented instead, and you get the `ToString`"] # [doc = " implementation for free."] # [doc = ""] # [doc = " [`Display`]: fmt::Display"] # [rustc_diagnostic_item = "ToString"] # [stable (feature = "rust1" , since = "1.0.0")] pub trait ToString { # [doc = " Converts the given value to a `String`."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let i = 5;"] # [doc = " let five = String::from(\"5\");"] # [doc = ""] # [doc = " assert_eq!(five, i.to_string());"] # [doc = " ```"] # [rustc_conversion_suggestion] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_diagnostic_item = "to_string_method"] fn to_string (& self) -> String ; }
};
}
