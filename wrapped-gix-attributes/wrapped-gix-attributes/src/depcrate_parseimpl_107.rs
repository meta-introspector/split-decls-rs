// Generated macro for impl_107 (impl)
macro_rules! Depcrate_parseimpl_107 {
() => {
// Module: crate::parse
// Provides: {"impl_107"}
// Dependencies: {}
impl < 'a > Iterator for Iter < 'a > { type Item = Result < AssignmentRef < 'a > , name :: Error > ; fn next (& mut self) -> Option < Self :: Item > { let attr = self . attrs . next () . filter (| a | ! a . is_empty ()) ? ; self . parse_attr (attr) . into () } }
};
}
