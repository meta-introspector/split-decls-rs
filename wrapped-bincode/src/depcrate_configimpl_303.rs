// Generated macro for impl_303 (impl)
macro_rules! Depcrate_configimpl_303 {
() => {
// Module: crate::config
// Provides: {"impl_303"}
// Dependencies: {}
impl < T > Config for T where T : InternalEndianConfig + InternalIntEncodingConfig + InternalLimitConfig + Copy + Clone , { fn endianness (& self) -> Endianness { < T as InternalEndianConfig > :: ENDIAN } fn int_encoding (& self) -> IntEncoding { < T as InternalIntEncodingConfig > :: INT_ENCODING } fn limit (& self) -> Option < usize > { < T as InternalLimitConfig > :: LIMIT } }
};
}
