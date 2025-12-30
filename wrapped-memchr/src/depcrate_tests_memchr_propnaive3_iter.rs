// Generated macro for naive3_iter (function)
macro_rules! Depcrate_tests_memchr_propnaive3_iter {
() => {
// Module: crate::tests::memchr::prop
// Provides: {"naive3_iter"}
// Dependencies: {}
# [cfg (not (miri))] pub (crate) fn naive3_iter < 'a > (n1 : u8 , n2 : u8 , n3 : u8 , haystack : & 'a [u8] ,) -> impl DoubleEndedIterator < Item = usize > + 'a { haystack . iter () . enumerate () . filter (move | & (_ , & b) | b == n1 || b == n2 || b == n3) . map (| t | t . 0) }
};
}
