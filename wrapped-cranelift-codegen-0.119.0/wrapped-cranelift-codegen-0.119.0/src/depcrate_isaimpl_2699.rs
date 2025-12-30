// Generated macro for impl_2699 (impl)
macro_rules! Depcrate_isaimpl_2699 {
() => {
// Module: crate::isa
// Provides: {"impl_2699"}
// Dependencies: {}
impl TargetFrontendConfig { # [doc = " Get the pointer type of this target."] pub fn pointer_type (self) -> ir :: Type { ir :: Type :: int (self . pointer_bits () as u16) . unwrap () } # [doc = " Get the width of pointers on this target, in units of bits."] pub fn pointer_bits (self) -> u8 { self . pointer_width . bits () } # [doc = " Get the width of pointers on this target, in units of bytes."] pub fn pointer_bytes (self) -> u8 { self . pointer_width . bytes () } }
};
}
