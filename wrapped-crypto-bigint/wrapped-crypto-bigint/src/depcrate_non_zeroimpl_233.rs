// Generated macro for impl_233 (impl)
macro_rules! Depcrate_non_zeroimpl_233 {
() => {
// Module: crate::non_zero
// Provides: {"impl_233"}
// Dependencies: {}
# [cfg (feature = "hybrid-array")] impl < T > NonZero < T > where T : ArrayEncoding + Zero , { # [doc = " Decode a non-zero integer from big endian bytes."] pub fn from_be_byte_array (bytes : ByteArray < T >) -> CtOption < Self > { Self :: new (T :: from_be_byte_array (bytes)) } # [doc = " Decode a non-zero integer from big endian bytes."] pub fn from_le_byte_array (bytes : ByteArray < T >) -> CtOption < Self > { Self :: new (T :: from_be_byte_array (bytes)) } }
};
}
