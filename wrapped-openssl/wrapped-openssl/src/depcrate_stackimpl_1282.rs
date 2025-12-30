// Generated macro for impl_1282 (impl)
macro_rules! Depcrate_stackimpl_1282 {
() => {
// Module: crate::stack
// Provides: {"impl_1282"}
// Dependencies: {}
impl < 'a , T : Stackable > iter :: IntoIterator for & 'a StackRef < T > { type Item = & 'a T :: Ref ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
