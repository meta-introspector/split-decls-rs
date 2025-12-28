macro_rules! impl_116 {
    () => {
        impl < S , U , F > Stream for FlatMap < S , U , F > where S : Stream , U : Stream , F : FnMut (S :: Item) -> U , { type Item = U :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { if let Some (inner) = this . inner_stream . as_mut () . as_pin_mut () { match ready ! (inner . poll_next (cx)) { Some (item) => return Poll :: Ready (Some (item)) , None => this . inner_stream . set (None) , } } match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (stream) => this . inner_stream . set (Some (stream)) , None => return Poll :: Ready (None) , } } } }
    };
}

impl_116!()