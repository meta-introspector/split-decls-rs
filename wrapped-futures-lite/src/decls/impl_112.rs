macro_rules! impl_112 {
    () => {
        impl < S : Stream > Stream for Fuse < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < S :: Item > > { let this = self . project () ; if * this . done { Poll :: Ready (None) } else { let next = ready ! (this . stream . poll_next (cx)) ; if next . is_none () { * this . done = true ; } Poll :: Ready (next) } } }
    };
}

impl_112!()