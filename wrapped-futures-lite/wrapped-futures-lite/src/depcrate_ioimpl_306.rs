// Generated macro for impl_306 (impl)
macro_rules! Depcrate_ioimpl_306 {
() => {
// Module: crate::io
// Provides: {"impl_306"}
// Dependencies: {}
impl < R : AsyncBufRead > Stream for Split < R > { type Item = Result < Vec < u8 > > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; let n = ready ! (read_until_internal (this . reader , cx , * this . delim , this . buf , this . read)) ? ; if n == 0 && this . buf . is_empty () { return Poll :: Ready (None) ; } if this . buf [this . buf . len () - 1] == * this . delim { this . buf . pop () ; } Poll :: Ready (Some (Ok (mem :: take (this . buf)))) } }
};
}
