// Generated macro for impl_149 (impl)
macro_rules! Depcrate_packed_patternimpl_149 {
() => {
// Module: crate::packed::pattern
// Provides: {"impl_149"}
// Dependencies: {}
impl < 'p > Iterator for PatternIter < 'p > { type Item = (PatternID , Pattern < 'p >) ; fn next (& mut self) -> Option < (PatternID , Pattern < 'p >) > { if self . i >= self . patterns . len () { return None ; } let id = self . patterns . order [self . i] ; let p = self . patterns . get (id) ; self . i += 1 ; Some ((id , p)) } }
};
}
