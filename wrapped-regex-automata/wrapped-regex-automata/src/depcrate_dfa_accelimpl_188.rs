// Generated macro for impl_188 (impl)
macro_rules! Depcrate_dfa_accelimpl_188 {
() => {
// Module: crate::dfa::accel
// Provides: {"impl_188"}
// Dependencies: {}
impl < 'a , A : AsRef < [AccelTy] > > Iterator for IterAccels < 'a , A > { type Item = Accel ; fn next (& mut self) -> Option < Accel > { let accel = self . accels . get (self . i) ? ; self . i += 1 ; Some (accel) } }
};
}
