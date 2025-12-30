// Generated macro for impl_1427 (impl)
macro_rules! Depcrate_resultimpl_1427 {
() => {
// Module: crate::result
// Provides: {"impl_1427"}
// Dependencies: {}
impl < 'a , T : Send , E > IntoParallelIterator for & 'a mut Result < T , E > { type Item = & 'a mut T ; type Iter = IterMut < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { IterMut { inner : self . as_mut () . ok () . into_par_iter () , } } }
};
}
