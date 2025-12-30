// Generated macro for impl_56 (impl)
macro_rules! Depcrate_simple_resetimpl_56 {
() => {
// Module: crate::simple_reset
// Provides: {"impl_56"}
// Dependencies: {}
impl < D : Digest + BlockSizeUser + Reset > Reset for SimpleHmacReset < D > { fn reset (& mut self) { Reset :: reset (& mut self . digest) ; self . digest . update (& self . ipad_key) ; } }
};
}
