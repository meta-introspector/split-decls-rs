// Generated macro for impl_95 (impl)
macro_rules! Depcrateimpl_95 {
() => {
// Module: crate
// Provides: {"impl_95"}
// Dependencies: {}
impl SuffixStrategy { fn is_match (& self , candidate : & Candidate < '_ >) -> bool { let path = candidate . path_suffix (self . longest) ; for m in self . matcher . find_overlapping_iter (path) { if m . end () == path . len () { return true ; } } false } fn matches_into (& self , candidate : & Candidate < '_ > , matches : & mut Vec < usize > ,) { let path = candidate . path_suffix (self . longest) ; for m in self . matcher . find_overlapping_iter (path) { if m . end () == path . len () { matches . push (self . map [m . pattern ()]) ; } } } }
};
}
