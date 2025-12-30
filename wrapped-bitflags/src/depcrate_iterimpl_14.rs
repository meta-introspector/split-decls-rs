// Generated macro for impl_14 (impl)
macro_rules! Depcrate_iterimpl_14 {
() => {
// Module: crate::iter
// Provides: {"impl_14"}
// Dependencies: {}
impl < B : Flags > Iterator for IterDefinedNames < B > { type Item = (& 'static str , B) ; fn next (& mut self) -> Option < Self :: Item > { while let Some (flag) = self . flags . get (self . idx) { self . idx += 1 ; if flag . is_named () { return Some ((flag . name () , B :: from_bits_retain (flag . value () . bits ()))) ; } } None } }
};
}
