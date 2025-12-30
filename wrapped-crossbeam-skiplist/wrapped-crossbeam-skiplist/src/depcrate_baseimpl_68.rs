// Generated macro for impl_68 (impl)
macro_rules! Depcrate_baseimpl_68 {
() => {
// Module: crate::base
// Provides: {"impl_68"}
// Dependencies: {}
impl < K , V > Iterator for IntoIter < K , V > { type Item = (K , V) ; fn next (& mut self) -> Option < (K , V) > { loop { if self . node . is_null () { return None ; } unsafe { let key = ptr :: read (& (* self . node) . key) ; let value = ptr :: read (& (* self . node) . value) ; let next = (* self . node) . tower [0] . load (Ordering :: Relaxed , epoch :: unprotected ()) ; Node :: dealloc (self . node) ; self . node = next . as_raw () as * mut Node < K , V > ; if next . tag () == 0 { return Some ((key , value)) ; } } } } }
};
}
