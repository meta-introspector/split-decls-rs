// Generated macro for inner (module)
macro_rules! Depcrate_util_memchrinner {
() => {
// Module: crate::util::memchr
// Provides: {"inner"}
// Dependencies: {}
# [cfg (not (feature = "perf-literal-substring"))] pub (super) mod inner { # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn memchr (n1 : u8 , haystack : & [u8]) -> Option < usize > { haystack . iter () . position (| & b | b == n1) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn memchr2 (n1 : u8 , n2 : u8 , haystack : & [u8]) -> Option < usize > { haystack . iter () . position (| & b | b == n1 || b == n2) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn memchr3 (n1 : u8 , n2 : u8 , n3 : u8 , haystack : & [u8] ,) -> Option < usize > { haystack . iter () . position (| & b | b == n1 || b == n2 || b == n3) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn memrchr (n1 : u8 , haystack : & [u8]) -> Option < usize > { haystack . iter () . rposition (| & b | b == n1) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn memrchr2 (n1 : u8 , n2 : u8 , haystack : & [u8]) -> Option < usize > { haystack . iter () . rposition (| & b | b == n1 || b == n2) } # [cfg_attr (feature = "perf-inline" , inline (always))] pub (crate) fn memrchr3 (n1 : u8 , n2 : u8 , n3 : u8 , haystack : & [u8] ,) -> Option < usize > { haystack . iter () . rposition (| & b | b == n1 || b == n2 || b == n3) } }
};
}
