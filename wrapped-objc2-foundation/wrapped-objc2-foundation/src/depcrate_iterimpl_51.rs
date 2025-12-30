// Generated macro for impl_51 (impl)
macro_rules! Depcrate_iterimpl_51 {
() => {
// Module: crate::iter
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'a , C : ? Sized + FastEnumerationHelper > IterUnchecked < 'a , C > { pub (crate) fn new (collection : & 'a C) -> Self { Self { helper : FastEnumeratorHelper :: new () , collection , # [cfg (debug_assertions)] mutations_state : None , } } }
};
}
