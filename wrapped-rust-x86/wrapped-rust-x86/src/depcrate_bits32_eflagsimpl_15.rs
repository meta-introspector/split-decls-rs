// Generated macro for impl_15 (impl)
macro_rules! Depcrate_bits32_eflagsimpl_15 {
() => {
// Module: crate::bits32::eflags
// Provides: {"impl_15"}
// Dependencies: {}
impl EFlags { # [doc = " Creates a new Flags entry. Ensures bit 1 is set."] pub const fn new () -> EFlags { EFlags :: FLAGS_A1 } # [doc = " Creates a new Flags with the given I/O privilege level."] pub const fn from_priv (iopl : Ring) -> EFlags { EFlags { bits : (iopl as u32) << 12 , } } }
};
}
