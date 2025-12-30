// Generated macro for impl_726 (impl)
macro_rules! Depcrate_types_valueimpl_726 {
() => {
// Module: crate::types::value
// Provides: {"impl_726"}
// Dependencies: {}
# [cfg (feature = "i128_blob")] impl From < i128 > for Value { # [inline] fn from (i : i128) -> Self { Self :: Blob (i128 :: to_be_bytes (i ^ (1_i128 << 127)) . to_vec ()) } }
};
}
