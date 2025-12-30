// Generated macro for impl_347 (impl)
macro_rules! Depcrate_re_traitimpl_347 {
() => {
// Module: crate::re_trait
// Provides: {"impl_347"}
// Dependencies: {}
impl < 't , R > Iterator for FindCaptures < 't , R > where R : RegularExpression , R :: Text : 't + AsRef < [u8] > { type Item = Vec < Slot > ; fn next (& mut self) -> Option < Vec < Slot > > { if self . 0 . last_end > self . 0 . text . as_ref () . len () { return None } let mut slots = vec ! [None ; self . 0 . re . slots_len ()] ; let (s , e) = match self . 0 . re . read_captures_at (& mut slots , self . 0 . text , self . 0 . last_end ,) { None => return None , Some ((s , e)) => (s , e) , } ; if s == e { self . 0 . last_end = self . 0 . re . next_after_empty (& self . 0 . text , e) ; if Some (e) == self . 0 . last_match { return self . next () ; } } else { self . 0 . last_end = e ; } self . 0 . last_match = Some (e) ; Some (slots) } }
};
}
