// Generated macro for impl_86 (impl)
macro_rules! Depcrate_stream_xml_readerimpl_86 {
() => {
// Module: crate::stream::xml_reader
// Provides: {"impl_86"}
// Dependencies: {}
impl < R : BufRead > Iterator for XmlReader < R > { type Item = Result < OwnedEvent , Error > ; fn next (& mut self) -> Option < Result < OwnedEvent , Error > > { if self . finished { return None ; } loop { match self . state . read_next (& mut self . buffer) { Ok (ReadResult :: XmlDecl) => { self . started = true ; } Ok (ReadResult :: Event (event)) => { self . started = true ; return Some (Ok (event)) ; } Ok (ReadResult :: Eof) => { self . started = true ; self . finished = true ; return None ; } Err (err) => { self . finished = true ; return Some (Err (err)) ; } } } } }
};
}
