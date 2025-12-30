// Generated macro for impl_93 (impl)
macro_rules! Depcrateimpl_93 {
() => {
// Module: crate
// Provides: {"impl_93"}
// Dependencies: {}
impl PrefixStrategy { fn is_match (& self , candidate : & Candidate < '_ >) -> bool { let path = candidate . path_prefix (self . longest) ; for m in self . matcher . find_overlapping_iter (path) { if m . start () == 0 { return true ; } } false } fn matches_into (& self , candidate : & Candidate < '_ > , matches : & mut Vec < usize > ,) { let path = candidate . path_prefix (self . longest) ; for m in self . matcher . find_overlapping_iter (path) { if m . start () == 0 { matches . push (self . map [m . pattern ()]) ; } } } }
};
}
