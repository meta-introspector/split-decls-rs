// Generated macro for impl_872 (impl)
macro_rules! Depcrate_unordimpl_872 {
() => {
// Module: crate::unord
// Provides: {"impl_872"}
// Dependencies: {}
impl < K : Hash + Eq , V > Extend < (K , V) > for UnordMap < K , V > { # [inline] fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { self . inner . extend (iter) } }
};
}
