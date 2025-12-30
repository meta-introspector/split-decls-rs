// Generated macro for impl_1322 (impl)
macro_rules! Depcrate_optionimpl_1322 {
() => {
// Module: crate::option
// Provides: {"impl_1322"}
// Dependencies: {}
impl < 'a , T : Sync > IntoParallelIterator for & 'a Option < T > { type Item = & 'a T ; type Iter = Iter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { Iter { inner : self . as_ref () . into_par_iter () , } } }
};
}
