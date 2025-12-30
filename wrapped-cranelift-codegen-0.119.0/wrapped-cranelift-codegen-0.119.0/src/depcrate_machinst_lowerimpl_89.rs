// Generated macro for impl_89 (impl)
macro_rules! Depcrate_machinst_lowerimpl_89 {
() => {
// Module: crate::machinst::lower
// Provides: {"impl_89"}
// Dependencies: {}
impl ValueUseState { # [doc = " Add one use."] fn inc (& mut self) { let new = match self { Self :: Unused => Self :: Once , Self :: Once | Self :: Multiple => Self :: Multiple , } ; * self = new ; } }
};
}
