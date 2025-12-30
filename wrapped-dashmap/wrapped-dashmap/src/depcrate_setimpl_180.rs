// Generated macro for impl_180 (impl)
macro_rules! Depcrate_setimpl_180 {
() => {
// Module: crate::set
// Provides: {"impl_180"}
// Dependencies: {}
impl < K : Eq + Hash , S : BuildHasher + Clone > Extend < K > for DashSet < K , S > { fn extend < T : IntoIterator < Item = K > > (& mut self , iter : T) { let iter = iter . into_iter () . map (| k | (k , ())) ; self . inner . extend (iter) } }
};
}
