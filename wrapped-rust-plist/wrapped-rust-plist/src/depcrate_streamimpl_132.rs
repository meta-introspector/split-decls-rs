// Generated macro for impl_132 (impl)
macro_rules! Depcrate_streamimpl_132 {
() => {
// Module: crate::stream
// Provides: {"impl_132"}
// Dependencies: {}
impl < R : Read + Seek > Iterator for Reader < R > { type Item = Result < OwnedEvent , Error > ; fn next (& mut self) -> Option < Result < OwnedEvent , Error > > { match self . 0 { ReaderInner :: Xml (ref mut parser) => parser . next () , ReaderInner :: Binary (ref mut parser) => parser . next () , ReaderInner :: Ascii (ref mut parser) => parser . next () , ReaderInner :: Uninitialized (ref mut reader) => { let reader = reader . take () . unwrap () ; self . init (reader) . transpose () } } } }
};
}
