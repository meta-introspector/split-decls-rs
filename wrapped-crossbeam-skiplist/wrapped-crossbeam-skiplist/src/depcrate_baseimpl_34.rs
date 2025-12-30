// Generated macro for impl_34 (impl)
macro_rules! Depcrate_baseimpl_34 {
() => {
// Module: crate::base
// Provides: {"impl_34"}
// Dependencies: {}
impl < K , V > Drop for SkipList < K , V > { fn drop (& mut self) { unsafe { let mut node = self . head [0] . load (Ordering :: Relaxed , epoch :: unprotected ()) . as_ref () ; while let Some (n) = node { let next = n . tower [0] . load (Ordering :: Relaxed , epoch :: unprotected ()) . as_ref () ; Node :: finalize (n) ; node = next ; } } } }
};
}
