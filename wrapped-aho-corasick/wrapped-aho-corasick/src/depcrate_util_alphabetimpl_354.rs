// Generated macro for impl_354 (impl)
macro_rules! Depcrate_util_alphabetimpl_354 {
() => {
// Module: crate::util::alphabet
// Provides: {"impl_354"}
// Dependencies: {}
impl Iterator for ByteClassIter { type Item = u8 ; fn next (& mut self) -> Option < u8 > { self . it . next () . map (| class | class . as_u8 ()) } }
};
}
