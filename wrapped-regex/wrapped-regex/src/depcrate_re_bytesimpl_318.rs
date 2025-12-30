// Generated macro for impl_318 (impl)
macro_rules! Depcrate_re_bytesimpl_318 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_318"}
// Dependencies: {}
impl < 'c , 't > Iterator for SubCaptures < 'c , 't > { type Item = Option < & 't [u8] > ; fn next (& mut self) -> Option < Option < & 't [u8] > > { if self . idx < self . caps . len () { self . idx += 1 ; Some (self . caps . at (self . idx - 1)) } else { None } } }
};
}
