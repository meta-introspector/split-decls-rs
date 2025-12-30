// Generated macro for ParseIntError (struct)
macro_rules! DepcrateParseIntError {
() => {
// Module: crate
// Provides: {"ParseIntError"}
// Dependencies: {}
# [doc = " An error which can be returned when parsing an integer."] # [doc = ""] # [doc = " This error is used as the error type for the `from_str_radix()` functions on ranged integer"] # [doc = " types, such as [`RangedI8::from_str_radix`]."] # [doc = ""] # [doc = " # Potential causes"] # [doc = ""] # [doc = " Among other causes, `ParseIntError` can be thrown because of leading or trailing whitespace"] # [doc = " in the string e.g., when it is obtained from the standard input."] # [doc = " Using the [`str::trim()`] method ensures that no whitespace remains before parsing."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use deranged::RangedI32;"] # [doc = " if let Err(e) = RangedI32::<0, 10>::from_str_radix(\"a12\", 10) {"] # [doc = "     println!(\"Failed conversion to RangedI32: {e}\");"] # [doc = " }"] # [doc = " ```"] # [allow (missing_copy_implementations)] # [derive (Debug , Clone , PartialEq , Eq)] pub struct ParseIntError { # [allow (clippy :: missing_docs_in_private_items)] kind : IntErrorKind , }
};
}
