// Generated macro for memrchr (function)
macro_rules! Depcrate_tests_memchr_naivememrchr {
() => {
// Module: crate::tests::memchr::naive
// Provides: {"memrchr"}
// Dependencies: {}
pub (crate) fn memrchr (n1 : u8 , haystack : & [u8]) -> Option < usize > { haystack . iter () . rposition (| & b | b == n1) }
};
}
