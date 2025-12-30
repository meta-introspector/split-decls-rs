// Generated macro for impl_63 (impl)
macro_rules! Depcrate_iterimpl_63 {
() => {
// Module: crate::iter
// Provides: {"impl_63"}
// Dependencies: {}
impl < 'a , C , E > IterWithBackingEnum < 'a , C , E > where C : ? Sized + FastEnumerationHelper , E : ? Sized + FastEnumerationHelper , { pub (crate) unsafe fn new (collection : & 'a C , enumerator : Retained < E >) -> Self { Self { helper : FastEnumeratorHelper :: new () , collection , enumerator , mutations_state : None , } } }
};
}
