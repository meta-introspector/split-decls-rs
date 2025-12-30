// Generated macro for impl_97 (impl)
macro_rules! Depcrateimpl_97 {
() => {
// Module: crate
// Provides: {"impl_97"}
// Dependencies: {}
impl RequiredExtensionStrategy { fn is_match (& self , candidate : & Candidate < '_ >) -> bool { if candidate . ext . is_empty () { return false ; } match self . 0 . get (candidate . ext . as_bytes ()) { None => false , Some (regexes) => { for & (_ , ref re) in regexes { if re . is_match (candidate . path . as_bytes ()) { return true ; } } false } } } # [inline (never)] fn matches_into (& self , candidate : & Candidate < '_ > , matches : & mut Vec < usize > ,) { if candidate . ext . is_empty () { return ; } if let Some (regexes) = self . 0 . get (candidate . ext . as_bytes ()) { for & (global_index , ref re) in regexes { if re . is_match (candidate . path . as_bytes ()) { matches . push (global_index) ; } } } } }
};
}
