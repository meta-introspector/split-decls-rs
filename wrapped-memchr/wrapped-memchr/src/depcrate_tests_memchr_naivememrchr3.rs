// Generated macro for memrchr3 (function)
macro_rules! Depcrate_tests_memchr_naivememrchr3 {
() => {
// Module: crate::tests::memchr::naive
// Provides: {"memrchr3"}
// Dependencies: {}
pub (crate) fn memrchr3 (n1 : u8 , n2 : u8 , n3 : u8 , haystack : & [u8] ,) -> Option < usize > { haystack . iter () . rposition (| & b | b == n1 || b == n2 || b == n3) }
};
}
