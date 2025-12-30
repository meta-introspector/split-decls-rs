// Generated macro for impl_273 (impl)
macro_rules! Depcrate_utf8impl_273 {
() => {
// Module: crate::utf8
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'a > Iterator for Chars < 'a > { type Item = char ; # [inline] fn next (& mut self) -> Option < char > { let (ch , size) = decode_lossy (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [size ..] ; Some (ch) } }
};
}
