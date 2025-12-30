// Generated macro for memchr (function)
macro_rules! Depcrate_tests_memchr_naivememchr {
() => {
// Module: crate::tests::memchr::naive
// Provides: {"memchr"}
// Dependencies: {}
pub (crate) fn memchr (n1 : u8 , haystack : & [u8]) -> Option < usize > { haystack . iter () . position (| & b | b == n1) }
};
}
