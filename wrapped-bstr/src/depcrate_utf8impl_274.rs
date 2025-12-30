// Generated macro for impl_274 (impl)
macro_rules! Depcrate_utf8impl_274 {
() => {
// Module: crate::utf8
// Provides: {"impl_274"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for Chars < 'a > { # [inline] fn next_back (& mut self) -> Option < char > { let (ch , size) = decode_last_lossy (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [.. self . bs . len () - size] ; Some (ch) } }
};
}
