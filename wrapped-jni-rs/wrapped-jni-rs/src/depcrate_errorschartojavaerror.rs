// Generated macro for CharToJavaError (struct)
macro_rules! Depcrate_errorsCharToJavaError {
() => {
// Module: crate::errors
// Provides: {"CharToJavaError"}
// Dependencies: {}
# [doc = " Raised by [`char_to_java`] and the implementation of `TryFrom<char>` for [`JValue`] / [`JValueOwned`] when a Rust [`char`] is not representable as a Java `char`."] # [doc = ""] # [doc = " See [`char_to_java`] for more information."] # [derive (Debug , Error)] # [error ("The code point U+{char_as_u32:X} {char:?} cannot be converted to a Java `char`, because it is not representable as a single UTF-16 unit." , char_as_u32 = u32 :: from (* char))] pub struct CharToJavaError { # [doc = " The character that could not be converted."] pub char : char , }
};
}
