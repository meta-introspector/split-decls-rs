// Generated macro for impl_278 (impl)
macro_rules! Depcrate_utf8impl_278 {
() => {
// Module: crate::utf8
// Provides: {"impl_278"}
// Dependencies: {}
impl < 'a > DoubleEndedIterator for CharIndices < 'a > { # [inline] fn next_back (& mut self) -> Option < (usize , usize , char) > { let (ch , size) = decode_last_lossy (self . bs) ; if size == 0 { return None ; } self . bs = & self . bs [.. self . bs . len () - size] ; self . reverse_index -= size ; Some ((self . reverse_index , self . reverse_index + size , ch)) } }
};
}
