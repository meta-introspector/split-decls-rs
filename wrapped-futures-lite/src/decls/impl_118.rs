macro_rules! impl_118 {
    () => {
        impl < S , U > Stream for Flatten < S > where S : Stream < Item = U > , U : Stream , { type Item = U :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { if let Some (inner) = this . inner_stream . as_mut () . as_pin_mut () { match ready ! (inner . poll_next (cx)) { Some (item) => return Poll :: Ready (Some (item)) , None => this . inner_stream . set (None) , } } match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (inner) => this . inner_stream . set (Some (inner)) , None => return Poll :: Ready (None) , } } } }
    };
}

impl_118!();