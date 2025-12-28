macro_rules! impl_133 {
    () => {
        impl < S : Stream > Stream for Take < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < S :: Item > > { let this = self . project () ; if * this . n == 0 { Poll :: Ready (None) } else { let next = ready ! (this . stream . poll_next (cx)) ; match next { Some (_) => * this . n -= 1 , None => * this . n = 0 , } Poll :: Ready (next) } } }
    };
}

impl_133!();