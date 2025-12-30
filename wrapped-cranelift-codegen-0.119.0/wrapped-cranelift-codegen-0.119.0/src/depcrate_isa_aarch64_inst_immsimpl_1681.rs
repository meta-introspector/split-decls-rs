// Generated macro for impl_1681 (impl)
macro_rules! Depcrate_isa_aarch64_inst_immsimpl_1681 {
() => {
// Module: crate::isa::aarch64::inst::imms
// Provides: {"impl_1681"}
// Dependencies: {}
impl NZCV { # [doc = " Create a new NZCV flags representation."] pub fn new (n : bool , z : bool , c : bool , v : bool) -> NZCV { NZCV { n , z , c , v } } # [doc = " Bits for encoding."] pub fn bits (& self) -> u32 { (u32 :: from (self . n) << 3) | (u32 :: from (self . z) << 2) | (u32 :: from (self . c) << 1) | u32 :: from (self . v) } }
};
}
