// Generated macro for char_to_java_int (function)
macro_rules! Depcrate_jvaluechar_to_java_int {
() => {
// Module: crate::jvalue
// Provides: {"char_to_java_int"}
// Dependencies: {}
# [doc = " Converts a Rust `char` to a Java `int`."] # [doc = ""] # [doc = " This is the form expected or produced by certain Java APIs that process UTF-32 units, such as [`String.codePointAt`]."] # [doc = ""] # [doc = " As discussed in [`char_to_java`], Rust `char` cannot always be converted to Java `char`, but can always be converted to Java `int`. This is the recommended way to pass a Rust `char` to Java code."] # [doc = ""] # [doc = " # See Also"] # [doc = ""] # [doc = " * [`JValue::int_from_char`], a wrapper for this function that returns [`JValue::Int`]"] # [doc = " * [`char_from_java_int`], the opposite of this function"] # [doc = " * [`char_to_java`], an alternative to this function that converts to Java `char` but is likely to fail"] # [doc = ""] # [doc = " [`String.codePointAt`]: https://docs.oracle.com/en/java/javase/17/docs/api/java.base/java/lang/String.html#codePointAt(int)"] pub fn char_to_java_int (char : char) -> jint { u32 :: from (char) as jint }
};
}
