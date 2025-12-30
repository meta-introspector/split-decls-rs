// Generated macro for impl_416 (impl)
macro_rules! Depcrate_hash_setimpl_416 {
() => {
// Module: crate::hash::set
// Provides: {"impl_416"}
// Dependencies: {}
impl < 'a , A > Iterator for Iter < 'a , A > where A : 'a , { type Item = & 'a A ; fn next (& mut self) -> Option < Self :: Item > { self . it . next () . map (| (v , _) | & v . 0) } fn size_hint (& self) -> (usize , Option < usize >) { self . it . size_hint () } }
};
}
