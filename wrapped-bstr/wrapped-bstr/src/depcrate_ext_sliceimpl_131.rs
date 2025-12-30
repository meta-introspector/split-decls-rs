// Generated macro for impl_131 (impl)
macro_rules! Depcrate_ext_sliceimpl_131 {
() => {
// Module: crate::ext_slice
// Provides: {"impl_131"}
// Dependencies: {}
impl < 'a > Iterator for LinesWithTerminator < 'a > { type Item = & 'a [u8] ; # [inline] fn next (& mut self) -> Option < & 'a [u8] > { match self . bytes . find_byte (b'\n') { None if self . bytes . is_empty () => None , None => { let line = self . bytes ; self . bytes = b"" ; Some (line) } Some (end) => { let line = & self . bytes [..= end] ; self . bytes = & self . bytes [end + 1 ..] ; Some (line) } } } }
};
}
