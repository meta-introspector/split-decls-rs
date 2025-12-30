// Generated macro for impl_86 (impl)
macro_rules! Depcrate_valueimpl_86 {
() => {
// Module: crate::value
// Provides: {"impl_86"}
// Dependencies: {}
impl < 'a , 'b > PartialEq < Value < 'b > > for Value < 'a > { # [inline] fn eq (& self , other : & Value < 'b >) -> bool { let left_content_chars = ContentChars :: from_string_unchecked (self . source) ; let right_content_chars = ContentChars :: from_string_unchecked (other . source) ; if self . ascii_case_insensitive || other . ascii_case_insensitive { left_content_chars . eq_ignore_ascii_case (& right_content_chars) } else { left_content_chars == right_content_chars } } }
};
}
