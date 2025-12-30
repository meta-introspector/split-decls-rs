// Generated macro for impl_28 (impl)
macro_rules! Depcrate_block_apiimpl_28 {
() => {
// Module: crate::block_api
// Provides: {"impl_28"}
// Dependencies: {}
impl < D : EagerHash > Reset for HmacResetCore < D > { # [inline (always)] fn reset (& mut self) { self . digest = self . ipad_digest . clone () ; } }
};
}
