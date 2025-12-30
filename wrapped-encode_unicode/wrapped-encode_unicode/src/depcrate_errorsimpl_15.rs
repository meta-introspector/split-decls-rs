// Generated macro for impl_15 (impl)
macro_rules! Depcrate_errorsimpl_15 {
() => {
// Module: crate::errors
// Provides: {"impl_15"}
// Dependencies: {}
impl CodepointError { # [doc = " Get the range of values for which this error would be given."] pub const fn error_range (self) -> RangeInclusive < u32 > { match self { Utf16Reserved => 0xd8_00 ..= 0xdf_ff , TooHigh => 0x00_10_ff_ff ..= 0xff_ff_ff_ff , } } }
};
}
