// Generated macro for memchr2 (function)
macro_rules! Depcrate_tests_memchr_naivememchr2 {
() => {
// Module: crate::tests::memchr::naive
// Provides: {"memchr2"}
// Dependencies: {}
pub (crate) fn memchr2 (n1 : u8 , n2 : u8 , haystack : & [u8]) -> Option < usize > { haystack . iter () . position (| & b | b == n1 || b == n2) }
};
}
