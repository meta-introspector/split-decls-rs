// Generated macro for impl_1325 (impl)
macro_rules! Depcrate_optionimpl_1325 {
() => {
// Module: crate::option
// Provides: {"impl_1325"}
// Dependencies: {}
impl < 'a , T : Send > IntoParallelIterator for & 'a mut Option < T > { type Item = & 'a mut T ; type Iter = IterMut < 'a , T > ; fn into_par_iter (self) -> Self :: Iter { IterMut { inner : self . as_mut () . into_par_iter () , } } }
};
}
