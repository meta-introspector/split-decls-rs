// Generated macro for naive1_iter (function)
macro_rules! Depcrate_tests_memchr_propnaive1_iter {
() => {
// Module: crate::tests::memchr::prop
// Provides: {"naive1_iter"}
// Dependencies: {}
# [cfg (not (miri))] pub (crate) fn naive1_iter < 'a > (n1 : u8 , haystack : & 'a [u8] ,) -> impl DoubleEndedIterator < Item = usize > + 'a { haystack . iter () . enumerate () . filter (move | & (_ , & b) | b == n1) . map (| t | t . 0) }
};
}
