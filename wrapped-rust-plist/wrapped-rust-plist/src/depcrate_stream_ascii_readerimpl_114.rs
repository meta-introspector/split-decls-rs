// Generated macro for impl_114 (impl)
macro_rules! Depcrate_stream_ascii_readerimpl_114 {
() => {
// Module: crate::stream::ascii_reader
// Provides: {"impl_114"}
// Dependencies: {}
impl < R : Read > Iterator for AsciiReader < R > { type Item = Result < OwnedEvent , Error > ; fn next (& mut self) -> Option < Result < OwnedEvent , Error > > { self . read_next () . transpose () } }
};
}
