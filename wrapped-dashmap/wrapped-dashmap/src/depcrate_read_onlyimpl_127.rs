// Generated macro for impl_127 (impl)
macro_rules! Depcrate_read_onlyimpl_127 {
() => {
// Module: crate::read_only
// Provides: {"impl_127"}
// Dependencies: {}
impl < K : Eq + Hash + fmt :: Debug , V : fmt :: Debug , S : BuildHasher + Clone > fmt :: Debug for ReadOnlyView < K , V , S > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { self . map . fmt (f) } }
};
}
