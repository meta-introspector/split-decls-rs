// Generated macro for impl_67 (impl)
macro_rules! Depcrate_baseimpl_67 {
() => {
// Module: crate::base
// Provides: {"impl_67"}
// Dependencies: {}
impl < K , V > Drop for IntoIter < K , V > { fn drop (& mut self) { while ! self . node . is_null () { unsafe { let next = (* self . node) . tower [0] . load (Ordering :: Relaxed , epoch :: unprotected ()) ; Node :: finalize (self . node) ; self . node = next . as_raw () as * mut Node < K , V > ; } } } }
};
}
