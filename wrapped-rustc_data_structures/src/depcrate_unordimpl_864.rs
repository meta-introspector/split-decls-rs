// Generated macro for impl_864 (impl)
macro_rules! Depcrate_unordimpl_864 {
() => {
// Module: crate::unord
// Provides: {"impl_864"}
// Dependencies: {}
impl < V : Hash + Eq > Extend < V > for UnordSet < V > { # [inline] fn extend < T : IntoIterator < Item = V > > (& mut self , iter : T) { self . inner . extend (iter) } }
};
}
