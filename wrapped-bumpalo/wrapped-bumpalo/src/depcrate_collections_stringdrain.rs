// Generated macro for Drain (struct)
macro_rules! Depcrate_collections_stringDrain {
() => {
// Module: crate::collections::string
// Provides: {"Drain"}
// Dependencies: {}
# [doc = " A draining iterator for `String`."] # [doc = ""] # [doc = " This struct is created by the [`String::drain`] method. See its"] # [doc = " documentation for more information."] pub struct Drain < 'a , 'bump > { # [doc = " Will be used as &'a mut String in the destructor"] string : * mut String < 'bump > , # [doc = " Start of part to remove"] start : usize , # [doc = " End of part to remove"] end : usize , # [doc = " Current remaining range to remove"] iter : Chars < 'a > , }
};
}
