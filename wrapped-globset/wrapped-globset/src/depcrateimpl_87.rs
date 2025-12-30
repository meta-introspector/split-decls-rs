// Generated macro for impl_87 (impl)
macro_rules! Depcrateimpl_87 {
() => {
// Module: crate
// Provides: {"impl_87"}
// Dependencies: {}
impl LiteralStrategy { fn new () -> LiteralStrategy { LiteralStrategy (fnv :: HashMap :: default ()) } fn add (& mut self , global_index : usize , lit : String) { self . 0 . entry (lit . into_bytes ()) . or_insert (vec ! []) . push (global_index) ; } fn is_match (& self , candidate : & Candidate < '_ >) -> bool { self . 0 . contains_key (candidate . path . as_bytes ()) } # [inline (never)] fn matches_into (& self , candidate : & Candidate < '_ > , matches : & mut Vec < usize > ,) { if let Some (hits) = self . 0 . get (candidate . path . as_bytes ()) { matches . extend (hits) ; } } }
};
}
