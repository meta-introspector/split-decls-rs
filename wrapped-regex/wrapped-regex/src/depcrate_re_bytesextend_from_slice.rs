// Generated macro for extend_from_slice (function)
macro_rules! Depcrate_re_bytesextend_from_slice {
() => {
// Module: crate::re_bytes
// Provides: {"extend_from_slice"}
// Dependencies: {}
# [doc = " This hopefully has the same performance characteristics as"] # [doc = " Vec::extend_from_slice (which was introduced in Rust 1.6), but works on"] # [doc = " Rust 1.3."] # [doc = ""] # [doc = " N.B. Remove this once we do a semver bump. At that point, we'll bump"] # [doc = " required Rust version to at least 1.6."] fn extend_from_slice (dst : & mut Vec < u8 > , src : & [u8]) { dst . reserve (src . len ()) ; let dst_len = dst . len () ; unsafe { dst . set_len (dst_len + src . len ()) ; } let mut dst = & mut dst [dst_len .. dst_len + src . len ()] ; for i in 0 .. src . len () { dst [i] = src [i] ; } }
};
}
