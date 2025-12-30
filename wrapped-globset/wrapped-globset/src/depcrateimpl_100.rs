// Generated macro for impl_100 (impl)
macro_rules! Depcrateimpl_100 {
() => {
// Module: crate
// Provides: {"impl_100"}
// Dependencies: {}
impl RegexSetStrategy { fn is_match (& self , candidate : & Candidate < '_ >) -> bool { self . matcher . is_match (candidate . path . as_bytes ()) } fn matches_into (& self , candidate : & Candidate < '_ > , matches : & mut Vec < usize > ,) { let input = regex_automata :: Input :: new (candidate . path . as_bytes ()) ; let mut patset = self . patset . get () ; patset . clear () ; self . matcher . which_overlapping_matches (& input , & mut patset) ; for i in patset . iter () { matches . push (self . map [i]) ; } PoolGuard :: put (patset) ; } }
};
}
