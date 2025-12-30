// Generated macro for memchr (function)
macro_rules! Depcrate_iomemchr {
() => {
// Module: crate::io
// Provides: {"memchr"}
// Dependencies: {}
# [doc = " Unoptimized memchr fallback."] # [cfg (not (feature = "memchr"))] fn memchr (needle : u8 , haystack : & [u8]) -> Option < usize > { haystack . iter () . position (| & b | b == needle) }
};
}
