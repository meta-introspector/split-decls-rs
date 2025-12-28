macro_rules! impl_139 {
    () => {
        impl < S : Stream > Stream for Skip < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (v) => match * this . n { 0 => return Poll :: Ready (Some (v)) , _ => * this . n -= 1 , } , None => return Poll :: Ready (None) , } } } }
    };
}

impl_139!();