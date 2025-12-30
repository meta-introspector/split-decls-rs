// Generated macro for write_cow_string (function)
macro_rules! Depcrate_utilswrite_cow_string {
() => {
// Module: crate::utils
// Provides: {"write_cow_string"}
// Dependencies: {}
# [allow (clippy :: ptr_arg)] pub fn write_cow_string (f : & mut Formatter , cow_string : & Cow < [u8] >) -> fmt :: Result { match cow_string { Cow :: Owned (s) => { write ! (f , "Owned(") ? ; write_byte_string (f , s) ? ; } Cow :: Borrowed (s) => { write ! (f , "Borrowed(") ? ; write_byte_string (f , s) ? ; } } write ! (f , ")") }
};
}
