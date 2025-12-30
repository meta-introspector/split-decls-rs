// Generated macro for impl_14 (impl)
macro_rules! Depcrateimpl_14 {
() => {
// Module: crate
// Provides: {"impl_14"}
// Dependencies: {}
impl Iterator for SinSignal { type Item = (f64 , f64) ; fn next (& mut self) -> Option < Self :: Item > { let point = (self . x , (self . x * 1.0 / self . period) . sin () * self . scale) ; self . x += self . interval ; Some (point) } }
};
}
