// Generated macro for impl_344 (impl)
macro_rules! Depcrate_re_traitimpl_344 {
() => {
// Module: crate::re_trait
// Provides: {"impl_344"}
// Dependencies: {}
impl < 't , R > Iterator for FindMatches < 't , R > where R : RegularExpression , R :: Text : 't + AsRef < [u8] > { type Item = (usize , usize) ; fn next (& mut self) -> Option < (usize , usize) > { if self . last_end > self . text . as_ref () . len () { return None ; } let (s , e) = match self . re . find_at (self . text , self . last_end) { None => return None , Some ((s , e)) => (s , e) , } ; if s == e { self . last_end = self . re . next_after_empty (& self . text , e) ; if Some (e) == self . last_match { return self . next () ; } } else { self . last_end = e ; } self . last_match = Some (e) ; Some ((s , e)) } }
};
}
