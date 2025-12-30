// Generated macro for impl_58 (impl)
macro_rules! Depcrate_dbimpl_58 {
() => {
// Module: crate::db
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a > Iterator for Names < 'a > { type Item = & 'a str ; fn next (& mut self) -> Option < & 'a str > { let mut i = self . position ; while i < self . database . 0 . len () { let lhs = self . database . 0 [i] . 0 ; if lhs . ber . eq (& self . oid . ber) { self . position = i + 1 ; return Some (self . database . 0 [i] . 1) ; } i += 1 ; } None } }
};
}
