// Generated macro for impl_1285 (impl)
macro_rules! Depcrate_stackimpl_1285 {
() => {
// Module: crate::stack
// Provides: {"impl_1285"}
// Dependencies: {}
impl < 'a , T : Stackable > iter :: IntoIterator for & 'a mut Stack < T > { type Item = & 'a mut T :: Ref ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
};
}
