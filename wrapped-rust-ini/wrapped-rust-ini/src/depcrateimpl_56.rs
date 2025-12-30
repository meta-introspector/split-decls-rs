// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < 'a > Iterator for SectionIterMut < 'a > { type Item = (Option < & 'a str > , & 'a mut Properties) ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| (k , v) | (k . as_ref () . map (| s | s . as_str ()) , v)) } fn size_hint (& self) -> (usize , Option < usize >) { self . inner . size_hint () } }
};
}
