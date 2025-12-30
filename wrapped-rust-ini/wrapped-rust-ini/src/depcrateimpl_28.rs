// Generated macro for impl_28 (impl)
macro_rules! Depcrateimpl_28 {
() => {
// Module: crate
// Provides: {"impl_28"}
// Dependencies: {}
impl < 'a > Iterator for PropertyIterMut < 'a > { type Item = (& 'a str , & 'a mut String) ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (k , v) | (k . as_ref () , v)) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
