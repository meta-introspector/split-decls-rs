// Generated macro for impl_309 (impl)
macro_rules! Depcrate_eventsimpl_309 {
() => {
// Module: crate::events
// Provides: {"impl_309"}
// Dependencies: {}
impl < 'a > Iterator for CDataIterator < 'a > { type Item = BytesCData < 'a > ; fn next (& mut self) -> Option < BytesCData < 'a > > { self . inner . next () . map (| slice | BytesCData :: wrap (slice . as_bytes () , Decoder :: utf8 ())) } }
};
}
