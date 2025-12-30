// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < T : Internable + ? Sized > InternStorage < T > { fn get (& self) -> & InternMap < T > { self . map . get_or_init (| | DashMap :: < Arc < T > , () , BuildHasherDefault < FxHasher > > :: default ()) } }
};
}
