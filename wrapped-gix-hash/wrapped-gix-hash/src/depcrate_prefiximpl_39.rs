// Generated macro for impl_39 (impl)
macro_rules! Depcrate_prefiximpl_39 {
() => {
// Module: crate::prefix
// Provides: {"impl_39"}
// Dependencies: {}
# [doc = " Create an instance from the given hexadecimal prefix, e.g. `35e77c16` would yield a `Prefix`"] # [doc = " with `hex_len()` = 8."] impl TryFrom < & str > for Prefix { type Error = from_hex :: Error ; fn try_from (value : & str) -> Result < Self , Self :: Error > { Prefix :: from_hex (value) } }
};
}
