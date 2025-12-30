// Generated macro for impl_102 (impl)
macro_rules! Depcrate_sourcesimpl_102 {
() => {
// Module: crate::sources
// Provides: {"impl_102"}
// Dependencies: {}
impl < 'a > Iterator for ByteLines < 'a > { type Item = & 'a [u8] ; fn next (& mut self) -> Option < Self :: Item > { if self . 0 . is_empty () { return None ; } let line_len = memchr (b'\n' , self . 0) . map_or (self . 0 . len () , | len | len + 1) ; let (line , rem) = self . 0 . split_at (line_len) ; self . 0 = rem ; Some (line) } }
};
}
