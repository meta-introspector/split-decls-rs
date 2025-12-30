// Generated macro for impl_261 (impl)
macro_rules! Depcrateimpl_261 {
() => {
// Module: crate
// Provides: {"impl_261"}
// Dependencies: {}
impl < K : Eq + Hash , V , S : BuildHasher + Clone > Extend < (K , V) > for DashMap < K , V , S > { fn extend < I : IntoIterator < Item = (K , V) > > (& mut self , intoiter : I) { for pair in intoiter . into_iter () { self . insert (pair . 0 , pair . 1) ; } } }
};
}
