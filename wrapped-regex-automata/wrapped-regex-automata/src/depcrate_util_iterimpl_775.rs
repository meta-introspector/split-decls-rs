// Generated macro for impl_775 (impl)
macro_rules! Depcrate_util_iterimpl_775 {
() => {
// Module: crate::util::iter
// Provides: {"impl_775"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'h , F > TryCapturesIter < 'h , F > { # [doc = " Return an infallible version of this iterator."] # [doc = ""] # [doc = " Any item yielded that corresponds to an error results in a panic. This"] # [doc = " is useful if your underlying regex engine is configured in a way that"] # [doc = " it is guaranteed to never return an error."] pub fn infallible (self) -> CapturesIter < 'h , F > { CapturesIter (self) } }
};
}
