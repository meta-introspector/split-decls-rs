// Generated macro for impl_2333 (impl)
macro_rules! Depcrate_io_linesimpl_2333 {
() => {
// Module: crate::io::lines
// Provides: {"impl_2333"}
// Dependencies: {}
impl < R : AsyncBufRead > Stream for Lines < R > { type Item = io :: Result < String > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; let n = ready ! (read_line_internal (this . reader , cx , this . buf , this . bytes , this . read)) ? ; * this . read = 0 ; if n == 0 && this . buf . is_empty () { return Poll :: Ready (None) ; } if this . buf . ends_with ('\n') { this . buf . pop () ; if this . buf . ends_with ('\r') { this . buf . pop () ; } } Poll :: Ready (Some (Ok (mem :: take (this . buf)))) } }
};
}
