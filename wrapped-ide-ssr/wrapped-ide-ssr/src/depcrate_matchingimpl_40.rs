// Generated macro for impl_40 (impl)
macro_rules! Depcrate_matchingimpl_40 {
() => {
// Module: crate::matching
// Provides: {"impl_40"}
// Dependencies: {}
impl Iterator for PatternIterator { type Item = SyntaxElement ; fn next (& mut self) -> Option < SyntaxElement > { self . iter . find (| element | ! element . kind () . is_trivia ()) } }
};
}
