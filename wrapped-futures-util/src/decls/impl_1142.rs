macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_1142 {
    () => {
        deps!();
        impl < R : AsyncBufRead > Stream for Lines < R > { type Item = io :: Result < String > ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; let n = ready ! (read_line_internal (this . reader , cx , this . buf , this . bytes , this . read)) ? ; * this . read = 0 ; if n == 0 && this . buf . is_empty () { return Poll :: Ready (None) ; } if this . buf . ends_with ('\n') { this . buf . pop () ; if this . buf . ends_with ('\r') { this . buf . pop () ; } } Poll :: Ready (Some (Ok (mem :: take (this . buf)))) } }
    };
}

impl_1142!();