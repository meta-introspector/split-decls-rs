// Generated macro for impl_153 (impl)
macro_rules! Depcrate_traitsimpl_153 {
() => {
// Module: crate::traits
// Provides: {"impl_153"}
// Dependencies: {}
impl digest :: ExtendableOutputReset for Hasher { # [inline] fn finalize_xof_reset (& mut self) -> Self :: Reader { let reader = Hasher :: finalize_xof (self) ; self . reset () ; reader } }
};
}
