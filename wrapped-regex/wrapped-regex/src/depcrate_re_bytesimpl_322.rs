// Generated macro for impl_322 (impl)
macro_rules! Depcrate_re_bytesimpl_322 {
() => {
// Module: crate::re_bytes
// Provides: {"impl_322"}
// Dependencies: {}
impl < 'c , 't > Iterator for SubCapturesNamed < 'c , 't > { type Item = (& 'c str , Option < & 't [u8] >) ; fn next (& mut self) -> Option < (& 'c str , Option < & 't [u8] >) > { self . names . next () . map (| (name , & pos) | (& * * name , self . caps . at (pos))) } }
};
}
