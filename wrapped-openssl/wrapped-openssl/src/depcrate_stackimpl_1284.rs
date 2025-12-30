// Generated macro for impl_1284 (impl)
macro_rules! Depcrate_stackimpl_1284 {
() => {
// Module: crate::stack
// Provides: {"impl_1284"}
// Dependencies: {}
impl < 'a , T : Stackable > iter :: IntoIterator for & 'a Stack < T > { type Item = & 'a T :: Ref ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
