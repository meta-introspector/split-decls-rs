// Generated macro for impl_54 (impl)
macro_rules! Depcrate_iterimpl_54 {
() => {
// Module: crate::iter
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'a , C : ? Sized + FastEnumerationHelper > Iter < 'a , C > { pub (crate) fn new (collection : & 'a C) -> Self { Self { helper : FastEnumeratorHelper :: new () , collection , mutations_state : None , } } }
};
}
