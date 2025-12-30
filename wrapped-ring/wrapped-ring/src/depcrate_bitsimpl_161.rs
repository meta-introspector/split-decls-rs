// Generated macro for impl_161 (impl)
macro_rules! Depcrate_bitsimpl_161 {
() => {
// Module: crate::bits
// Provides: {"impl_161"}
// Dependencies: {}
impl TryFrom < BitLength < u64 > > for BitLength < core :: num :: NonZeroU64 > { type Error = < core :: num :: NonZeroU64 as TryFrom < u64 > > :: Error ; fn try_from (BitLength (value) : BitLength < u64 >) -> Result < Self , Self :: Error > { value . try_into () . map (BitLength) } }
};
}
