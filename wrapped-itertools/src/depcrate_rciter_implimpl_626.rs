// Generated macro for impl_626 (impl)
macro_rules! Depcrate_rciter_implimpl_626 {
() => {
// Module: crate::rciter_impl
// Provides: {"impl_626"}
// Dependencies: {}
# [doc = " Return an iterator from `&RcIter<I>` (by simply cloning it)."] impl < I > IntoIterator for & RcIter < I > where I : Iterator , { type Item = I :: Item ; type IntoIter = RcIter < I > ; fn into_iter (self) -> RcIter < I > { self . clone () } }
};
}
