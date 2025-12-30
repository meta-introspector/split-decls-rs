// Generated macro for impl_88 (impl)
macro_rules! Depcrate_operatorsimpl_88 {
() => {
// Module: crate::operators
// Provides: {"impl_88"}
// Dependencies: {}
impl ops :: DivAssign < f64 > for Expression { fn div_assign (& mut self , v : f64) { self . constant /= v ; for t in & mut self . terms { * t = * t / v ; } } }
};
}
