// Generated macro for impl_96 (impl)
macro_rules! Depcrate_xof_fixedimpl_96 {
() => {
// Module: crate::xof_fixed
// Provides: {"impl_96"}
// Dependencies: {}
impl < T : ExtendableOutput + Reset , S : ArraySize > Reset for XofFixedWrapper < T , S > { fn reset (& mut self) { self . hash . reset () ; } }
};
}
