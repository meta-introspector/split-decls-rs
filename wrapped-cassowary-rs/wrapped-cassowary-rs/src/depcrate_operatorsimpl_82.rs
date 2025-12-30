// Generated macro for impl_82 (impl)
macro_rules! Depcrate_operatorsimpl_82 {
() => {
// Module: crate::operators
// Provides: {"impl_82"}
// Dependencies: {}
impl ops :: MulAssign < f64 > for Expression { fn mul_assign (& mut self , v : f64) { self . constant *= v ; for t in & mut self . terms { * t = * t * v ; } } }
};
}
