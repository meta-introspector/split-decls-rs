macro_rules! impl_151 {
    () => {
        impl < S > Stream for Cycle < S > where S : Stream + Clone , { type Item = S :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match ready ! (self . as_mut () . project () . stream . as_mut () . poll_next (cx)) { Some (item) => Poll :: Ready (Some (item)) , None => { let new = self . as_mut () . orig . clone () ; self . as_mut () . project () . stream . set (new) ; self . project () . stream . poll_next (cx) } } } }
    };
}

impl_151!();