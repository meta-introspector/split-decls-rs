// Generated macro for impl_53 (impl)
macro_rules! Depcrateimpl_53 {
() => {
// Module: crate
// Provides: {"impl_53"}
// Dependencies: {}
impl < 'a > Iterator for SectionIter < 'a > { type Item = (Option < & 'a str > , & 'a Properties) ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (k , v) | (k . as_ref () . map (| s | s . as_str ()) , v)) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
