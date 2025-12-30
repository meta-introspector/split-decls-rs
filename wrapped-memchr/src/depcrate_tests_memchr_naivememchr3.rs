// Generated macro for memchr3 (function)
macro_rules! Depcrate_tests_memchr_naivememchr3 {
() => {
// Module: crate::tests::memchr::naive
// Provides: {"memchr3"}
// Dependencies: {}
pub (crate) fn memchr3 (n1 : u8 , n2 : u8 , n3 : u8 , haystack : & [u8] ,) -> Option < usize > { haystack . iter () . position (| & b | b == n1 || b == n2 || b == n3) }
};
}
