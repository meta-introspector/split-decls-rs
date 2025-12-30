// Generated macro for impl_87 (impl)
macro_rules! Depcrate_valueimpl_87 {
() => {
// Module: crate::value
// Provides: {"impl_87"}
// Dependencies: {}
impl < 'a > PartialEq < str > for Value < 'a > { fn eq (& self , other : & str) -> bool { if self . source . starts_with ('"') { let content_chars = ContentChars :: from_string_unchecked (self . source) ; if self . ascii_case_insensitive { content_chars . eq_ignore_ascii_case (other) } else { content_chars == other } } else if self . ascii_case_insensitive { self . source . eq_ignore_ascii_case (other) } else { self . source == other } } }
};
}
