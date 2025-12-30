// Generated macro for impl_837 (impl)
macro_rules! Depcrate_rc_retained_traitsimpl_837 {
() => {
// Module: crate::rc::retained_traits
// Provides: {"impl_837"}
// Dependencies: {}
impl < T : ? Sized + RetainedIntoIterator > IntoIterator for Retained < T > { type Item = < T as RetainedIntoIterator > :: Item ; type IntoIter = < T as RetainedIntoIterator > :: IntoIter ; # [inline] fn into_iter (self) -> Self :: IntoIter { T :: retained_into_iter (self) } }
};
}
