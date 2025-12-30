// Generated macro for impl_89 (impl)
macro_rules! Depcrateimpl_89 {
() => {
// Module: crate
// Provides: {"impl_89"}
// Dependencies: {}
impl BasenameLiteralStrategy { fn new () -> BasenameLiteralStrategy { BasenameLiteralStrategy (fnv :: HashMap :: default ()) } fn add (& mut self , global_index : usize , lit : String) { self . 0 . entry (lit . into_bytes ()) . or_insert (vec ! []) . push (global_index) ; } fn is_match (& self , candidate : & Candidate < '_ >) -> bool { if candidate . basename . is_empty () { return false ; } self . 0 . contains_key (candidate . basename . as_bytes ()) } # [inline (never)] fn matches_into (& self , candidate : & Candidate < '_ > , matches : & mut Vec < usize > ,) { if candidate . basename . is_empty () { return ; } if let Some (hits) = self . 0 . get (candidate . basename . as_bytes ()) { matches . extend (hits) ; } } }
};
}
