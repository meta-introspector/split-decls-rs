// Generated macro for impl_1978 (impl)
macro_rules! Depcrate_vecimpl_1978 {
() => {
// Module: crate::vec
// Provides: {"impl_1978"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] # [stable (feature = "rust1" , since = "1.0.0")] impl From < & str > for Vec < u8 > { # [doc = " Allocates a `Vec<u8>` and fills it with a UTF-8 string."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " assert_eq!(Vec::from(\"123\"), vec![b'1', b'2', b'3']);"] # [doc = " ```"] # [track_caller] fn from (s : & str) -> Vec < u8 > { From :: from (s . as_bytes ()) } }
};
}
