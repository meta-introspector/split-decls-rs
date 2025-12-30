// Generated macro for impl_25 (impl)
macro_rules! Depcrateimpl_25 {
() => {
// Module: crate
// Provides: {"impl_25"}
// Dependencies: {}
impl < 'a > Iterator for PropertyIter < 'a > { type Item = (& 'a str , & 'a str) ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (k , v) | (k . as_ref () , v . as_ref ())) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
