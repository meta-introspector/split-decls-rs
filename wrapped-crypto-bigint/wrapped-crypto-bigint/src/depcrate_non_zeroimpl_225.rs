// Generated macro for impl_225 (impl)
macro_rules! Depcrate_non_zeroimpl_225 {
() => {
// Module: crate::non_zero
// Provides: {"impl_225"}
// Dependencies: {}
impl < T > NonZero < T > where T : Encoding + Zero , { # [doc = " Decode from big endian bytes."] pub fn from_be_bytes (bytes : T :: Repr) -> CtOption < Self > { Self :: new (T :: from_be_bytes (bytes)) } # [doc = " Decode from little endian bytes."] pub fn from_le_bytes (bytes : T :: Repr) -> CtOption < Self > { Self :: new (T :: from_le_bytes (bytes)) } }
};
}
