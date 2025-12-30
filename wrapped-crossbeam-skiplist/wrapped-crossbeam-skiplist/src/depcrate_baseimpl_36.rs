// Generated macro for impl_36 (impl)
macro_rules! Depcrate_baseimpl_36 {
() => {
// Module: crate::base
// Provides: {"impl_36"}
// Dependencies: {}
impl < K , V > IntoIterator for SkipList < K , V > { type Item = (K , V) ; type IntoIter = IntoIter < K , V > ; fn into_iter (self) -> IntoIter < K , V > { unsafe { let front = self . head [0] . load (Ordering :: Relaxed , epoch :: unprotected ()) . as_raw () ; for level in 0 .. MAX_HEIGHT { self . head [level] . store (Shared :: null () , Ordering :: Relaxed) ; } IntoIter { node : front as * mut Node < K , V > , } } } }
};
}
