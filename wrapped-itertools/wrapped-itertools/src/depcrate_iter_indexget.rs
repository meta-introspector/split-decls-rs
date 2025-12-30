// Generated macro for get (function)
macro_rules! Depcrate_iter_indexget {
() => {
// Module: crate::iter_index
// Provides: {"get"}
// Dependencies: {}
pub fn get < I , R > (iter : I , index : R) -> R :: Output where I : IntoIterator , R : IteratorIndex < I :: IntoIter > , { index . index (iter . into_iter ()) }
};
}
