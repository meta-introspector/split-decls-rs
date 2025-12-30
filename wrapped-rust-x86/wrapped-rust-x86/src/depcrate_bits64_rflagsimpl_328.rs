// Generated macro for impl_328 (impl)
macro_rules! Depcrate_bits64_rflagsimpl_328 {
() => {
// Module: crate::bits64::rflags
// Provides: {"impl_328"}
// Dependencies: {}
impl RFlags { # [doc = " Creates a new Flags entry. Ensures bit 1 is set."] pub const fn new () -> RFlags { RFlags :: FLAGS_A1 } # [doc = " Creates a new Flags with the given I/O privilege level."] pub const fn from_priv (iopl : Ring) -> RFlags { RFlags { bits : (iopl as u64) << 12 , } } pub const fn from_raw (bits : u64) -> RFlags { RFlags { bits } } }
};
}
