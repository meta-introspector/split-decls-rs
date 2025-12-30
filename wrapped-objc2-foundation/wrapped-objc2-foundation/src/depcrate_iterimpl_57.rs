// Generated macro for impl_57 (impl)
macro_rules! Depcrate_iterimpl_57 {
() => {
// Module: crate::iter
// Provides: {"impl_57"}
// Dependencies: {}
impl < C : ? Sized + FastEnumerationHelper > IntoIter < C > { pub (crate) fn new (collection : Retained < C >) -> Self { Self { helper : FastEnumeratorHelper :: new () , collection , mutations_state : None , } } pub (crate) fn new_mutable < T > (collection : Retained < T >) -> Self where T : ClassType < Super = C > , C : Sized , { Self { helper : FastEnumeratorHelper :: new () , collection : unsafe { Retained :: cast_unchecked (collection) } , mutations_state : None , } } }
};
}
