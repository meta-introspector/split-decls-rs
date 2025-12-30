// Generated macro for impl_66 (impl)
macro_rules! Depcrateimpl_66 {
() => {
// Module: crate
// Provides: {"impl_66"}
// Dependencies: {}
impl < K : Clone + Eq + Hash , V , S : BuildHasher > Extend < (K , V) > for CLruCache < K , V , S > { fn extend < T : IntoIterator < Item = (K , V) > > (& mut self , iter : T) { for (k , v) in iter { self . put (k , v) ; } } }
};
}
