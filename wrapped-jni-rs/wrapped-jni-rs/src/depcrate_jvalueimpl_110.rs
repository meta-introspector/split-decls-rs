// Generated macro for impl_110 (impl)
macro_rules! Depcrate_jvalueimpl_110 {
() => {
// Module: crate::jvalue
// Provides: {"impl_110"}
// Dependencies: {}
# [doc = " Converts a Rust `char` to a Java `char`, if possible."] # [doc = ""] # [doc = " **Warning:** This conversion is likely to fail. Using it is not recommended. Prefer"] # [doc = " [`JValue::int_from_char`] where possible. See [`char_to_java`] for more information."] impl TryFrom < char > for JValueOwned < '_ > { type Error = CharToJavaError ; fn try_from (value : char) -> std :: result :: Result < Self , Self :: Error > { Ok (Self :: Char (char_to_java (value) ?)) } }
};
}
