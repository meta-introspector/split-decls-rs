// Generated macro for impl_881 (impl)
macro_rules! Depcrate_unordimpl_881 {
() => {
// Module: crate::unord
// Provides: {"impl_881"}
// Dependencies: {}
impl < T > Extend < T > for UnordBag < T > { fn extend < I : IntoIterator < Item = T > > (& mut self , iter : I) { self . inner . extend (iter) } }
};
}
