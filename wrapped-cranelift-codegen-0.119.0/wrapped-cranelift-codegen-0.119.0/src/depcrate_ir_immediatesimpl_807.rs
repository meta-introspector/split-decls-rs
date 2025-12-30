// Generated macro for impl_807 (impl)
macro_rules! Depcrate_ir_immediatesimpl_807 {
() => {
// Module: crate::ir::immediates
// Provides: {"impl_807"}
// Dependencies: {}
impl Uimm64 { # [doc = " Create a new `Uimm64` representing the unsigned number `x`."] pub fn new (x : u64) -> Self { Self (x) } # [doc = " Return self negated."] pub fn wrapping_neg (self) -> Self { Self (self . 0 . wrapping_neg ()) } }
};
}
