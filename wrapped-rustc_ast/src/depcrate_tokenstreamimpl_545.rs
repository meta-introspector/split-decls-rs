// Generated macro for impl_545 (impl)
macro_rules! Depcrate_tokenstreamimpl_545 {
() => {
// Module: crate::tokenstream
// Provides: {"impl_545"}
// Dependencies: {}
impl < 't > Iterator for TokenStreamIter < 't > { type Item = & 't TokenTree ; fn next (& mut self) -> Option < & 't TokenTree > { self . stream . 0 . get (self . index) . map (| tree | { self . index += 1 ; tree }) } }
};
}
