// Generated macro for impl_1317 (impl)
macro_rules! Depcrate_optionimpl_1317 {
() => {
// Module: crate::option
// Provides: {"impl_1317"}
// Dependencies: {}
impl < T : Send > IntoParallelIterator for Option < T > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { opt : self } } }
};
}
