// Generated macro for char_from_java_int (function)
macro_rules! Depcrate_jvaluechar_from_java_int {
() => {
// Module: crate::jvalue
// Provides: {"char_from_java_int"}
// Dependencies: {}
# [doc = " Converts a Java `int` to a Rust `char`."] # [doc = ""] # [doc = " This is the form expected or produced by certain Java APIs that process UTF-32 units, such as [`String.codePointAt`]."] # [doc = ""] # [doc = " As discussed in [`char_from_java`], Rust `char` cannot always be converted from Java `char`, but can always be converted from Java `int` (provided that the `int` contains a valid UTF-32 unit). This is the recommended way to receive a Rust `char` from Java code."] # [doc = ""] # [doc = " # See Also"] # [doc = ""] # [doc = " * [`JValue::i_char`], a wrapper for this function that unwraps [`JValue::Int`]"] # [doc = " * [`JValueOwned::i_char`], a wrapper for this function that unwraps [`JValueOwned::Int`]"] # [doc = " * [`char_to_java_int`], the opposite of this function"] # [doc = " * [`char_from_java`], an alternative to this function that converts from Java `char` but is likely to fail"] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " Returns an error if the Java `int` doesn't represent a valid UTF-32 unit."] # [doc = ""] # [doc = " [`String.codePointAt`]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/String.html#codePointAt(int)"] pub fn char_from_java_int (jint : jint) -> std :: result :: Result < char , CharTryFromError > { char :: try_from (jint as u32) }
};
}
