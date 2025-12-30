// Generated macro for impl_1420 (impl)
macro_rules! Depcrate_resultimpl_1420 {
() => {
// Module: crate::result
// Provides: {"impl_1420"}
// Dependencies: {}
impl < T : Send , E > IntoParallelIterator for Result < T , E > { type Item = T ; type Iter = IntoIter < T > ; fn into_par_iter (self) -> Self :: Iter { IntoIter { inner : self . ok () . into_par_iter () , } } }
};
}
