// Generated macro for impl_157 (impl)
macro_rules! Depcrate_bnimpl_157 {
() => {
// Module: crate::bn
// Provides: {"impl_157"}
// Dependencies: {}
impl MsbOption { # [doc = " The most significant bit of the number may be 0."] pub const MAYBE_ZERO : MsbOption = MsbOption (- 1) ; # [doc = " The most significant bit of the number must be 1."] pub const ONE : MsbOption = MsbOption (0) ; # [doc = " The most significant two bits of the number must be 1."] # [doc = ""] # [doc = " The number of bits in the product of two such numbers will always be exactly twice the"] # [doc = " number of bits in the original numbers."] pub const TWO_ONES : MsbOption = MsbOption (1) ; }
};
}
