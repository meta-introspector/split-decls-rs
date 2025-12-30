// Generated macro for impl_48 (impl)
macro_rules! Depcrate_stream_binary_readerimpl_48 {
() => {
// Module: crate::stream::binary_reader
// Provides: {"impl_48"}
// Dependencies: {}
impl < R : Read + Seek > Iterator for BinaryReader < R > { type Item = Result < OwnedEvent , Error > ; fn next (& mut self) -> Option < Result < OwnedEvent , Error > > { match self . read_next () { Ok (Some (event)) => Some (Ok (event)) , Err (err) => { self . stack . clear () ; Some (Err (err)) } Ok (None) => None , } } }
};
}
