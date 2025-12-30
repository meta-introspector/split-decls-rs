// Generated macro for impl_1283 (impl)
macro_rules! Depcrate_stackimpl_1283 {
() => {
// Module: crate::stack
// Provides: {"impl_1283"}
// Dependencies: {}
impl < 'a , T : Stackable > iter :: IntoIterator for & 'a mut StackRef < T > { type Item = & 'a mut T :: Ref ; type IntoIter = IterMut < 'a , T > ; fn into_iter (self) -> IterMut < 'a , T > { self . iter_mut () } }
};
}
