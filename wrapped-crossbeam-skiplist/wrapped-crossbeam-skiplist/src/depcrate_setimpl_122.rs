// Generated macro for impl_122 (impl)
macro_rules! Depcrate_setimpl_122 {
() => {
// Module: crate::set
// Provides: {"impl_122"}
// Dependencies: {}
impl < 'a , T > IntoIterator for & 'a SkipSet < T > where T : Ord , { type Item = Entry < 'a , T > ; type IntoIter = Iter < 'a , T > ; fn into_iter (self) -> Iter < 'a , T > { self . iter () } }
};
}
