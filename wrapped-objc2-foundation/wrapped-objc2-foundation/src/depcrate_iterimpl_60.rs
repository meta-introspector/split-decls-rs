// Generated macro for impl_60 (impl)
macro_rules! Depcrate_iterimpl_60 {
() => {
// Module: crate::iter
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , C , E > IterUncheckedWithBackingEnum < 'a , C , E > where C : ? Sized + FastEnumerationHelper , E : ? Sized + FastEnumerationHelper , { pub (crate) unsafe fn new (collection : & 'a C , enumerator : Retained < E >) -> Self { Self { helper : FastEnumeratorHelper :: new () , collection , enumerator , # [cfg (debug_assertions)] mutations_state : None , } } }
};
}
