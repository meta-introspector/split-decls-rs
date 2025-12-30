// Generated macro for impl_1424 (impl)
macro_rules! Depcrate_resultimpl_1424 {
() => {
// Module: crate::result
// Provides: {"impl_1424"}
// Dependencies: {}
impl < 'a , T : Sync , E > IntoParallelIterator for & 'a Result < T , E > { type Item = & 'a T ; type Iter = Iter < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { Iter { inner : self . as_ref () . ok () . into_par_iter () , } } }
};
}
