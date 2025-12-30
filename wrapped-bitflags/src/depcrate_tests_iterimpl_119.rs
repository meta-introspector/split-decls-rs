// Generated macro for impl_119 (impl)
macro_rules! Depcrate_tests_iterimpl_119 {
() => {
// Module: crate::tests::iter
// Provides: {"impl_119"}
// Dependencies: {}
impl < B : Flags > Iterator for IterNames < B > { type Item = (& 'static str , B) ; fn next (& mut self) -> Option < Self :: Item > { while let Some (flag) = self . flags . get (self . idx) { if self . remaining . is_empty () { return None ; } self . idx += 1 ; if flag . name () . is_empty () { continue ; } let bits = flag . value () . bits () ; if self . source . contains (B :: from_bits_retain (bits)) && self . remaining . intersects (B :: from_bits_retain (bits)) { self . remaining . remove (B :: from_bits_retain (bits)) ; return Some ((flag . name () , B :: from_bits_retain (bits))) ; } } None } }
};
}
