macro_rules! impl_160 {
    () => {
        impl < S : Stream > Future for LastFuture < S > { type Output = Option < S :: Item > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (new) => * this . last = Some (new) , None => return Poll :: Ready (this . last . take ()) , } } } }
    };
}

impl_160!()