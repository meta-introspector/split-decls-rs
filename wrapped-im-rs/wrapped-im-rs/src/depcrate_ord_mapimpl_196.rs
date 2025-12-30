// Generated macro for impl_196 (impl)
macro_rules! Depcrate_ord_mapimpl_196 {
() => {
// Module: crate::ord::map
// Provides: {"impl_196"}
// Dependencies: {}
impl < K , V > Debug for OrdMap < K , V > where K : Ord + Debug , V : Debug , { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { let mut d = f . debug_map () ; for (k , v) in self . iter () { d . entry (k , v) ; } d . finish () } }
};
}
