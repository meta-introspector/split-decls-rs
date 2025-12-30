// Generated macro for impl_838 (impl)
macro_rules! Depcrate_rc_retained_traitsimpl_838 {
() => {
// Module: crate::rc::retained_traits
// Provides: {"impl_838"}
// Dependencies: {}
impl < 'a , T : ? Sized > IntoIterator for & 'a Retained < T > where & 'a T : IntoIterator , { type Item = < & 'a T as IntoIterator > :: Item ; type IntoIter = < & 'a T as IntoIterator > :: IntoIter ; # [inline] fn into_iter (self) -> Self :: IntoIter { (& * * self) . into_iter () } }
};
}
