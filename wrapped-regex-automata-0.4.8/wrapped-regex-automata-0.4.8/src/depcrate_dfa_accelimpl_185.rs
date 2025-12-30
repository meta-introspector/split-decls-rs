// Generated macro for impl_185 (impl)
macro_rules! Depcrate_dfa_accelimpl_185 {
() => {
// Module: crate::dfa::accel
// Provides: {"impl_185"}
// Dependencies: {}
impl < 'a , A : AsRef < [AccelTy] > > Iterator for IterAccels < 'a , A > { type Item = Accel ; fn next (& mut self) -> Option < Accel > { let accel = self . accels . get (self . i) ? ; self . i += 1 ; Some (accel) } }
};
}
