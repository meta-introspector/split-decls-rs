// Generated macro for impl_91 (impl)
macro_rules! Depcrateimpl_91 {
() => {
// Module: crate
// Provides: {"impl_91"}
// Dependencies: {}
impl ExtensionStrategy { fn new () -> ExtensionStrategy { ExtensionStrategy (fnv :: HashMap :: default ()) } fn add (& mut self , global_index : usize , ext : String) { self . 0 . entry (ext . into_bytes ()) . or_insert (vec ! []) . push (global_index) ; } fn is_match (& self , candidate : & Candidate < '_ >) -> bool { if candidate . ext . is_empty () { return false ; } self . 0 . contains_key (candidate . ext . as_bytes ()) } # [inline (never)] fn matches_into (& self , candidate : & Candidate < '_ > , matches : & mut Vec < usize > ,) { if candidate . ext . is_empty () { return ; } if let Some (hits) = self . 0 . get (candidate . ext . as_bytes ()) { matches . extend (hits) ; } } }
};
}
