// Generated macro for impl_107 (impl)
macro_rules! Depcrate_parserimpl_107 {
() => {
// Module: crate::parser
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a > Iterator for SubtagIterator < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < Self :: Item > { let (s , res) = self . next_const () ; * self = s ; res } }
};
}
