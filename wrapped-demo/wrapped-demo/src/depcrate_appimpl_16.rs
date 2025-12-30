// Generated macro for impl_16 (impl)
macro_rules! Depcrate_appimpl_16 {
() => {
// Module: crate::app
// Provides: {"impl_16"}
// Dependencies: {}
impl Iterator for SinSignal { type Item = (f64 , f64) ; fn next (& mut self) -> Option < Self :: Item > { let point = (self . x , (self . x * 1.0 / self . period) . sin () * self . scale) ; self . x += self . interval ; Some (point) } }
};
}
