// Generated macro for Drain (struct)
macro_rules! Depcrate_stringDrain {
() => {
// Module: crate::string
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator for `String`."] # [doc = ""] # [doc = " This struct is created by the [`drain`] method on [`String`]. See its"] # [doc = " documentation for more."] # [doc = ""] # [doc = " [`drain`]: String::drain"] # [stable (feature = "drain" , since = "1.6.0")] pub struct Drain < 'a > { # [doc = " Will be used as &'a mut String in the destructor"] string : * mut String , # [doc = " Start of part to remove"] start : usize , # [doc = " End of part to remove"] end : usize , # [doc = " Current remaining range to remove"] iter : Chars < 'a > , }
};
}
