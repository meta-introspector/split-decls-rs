// Generated macro for memrchr2 (function)
macro_rules! Depcrate_tests_memchr_naivememrchr2 {
() => {
// Module: crate::tests::memchr::naive
// Provides: {"memrchr2"}
// Dependencies: {}
pub (crate) fn memrchr2 (n1 : u8 , n2 : u8 , haystack : & [u8]) -> Option < usize > { haystack . iter () . rposition (| & b | b == n1 || b == n2) }
};
}
