macro_rules! impl_99 {
    () => {
        impl < S , C > Future for CollectFuture < S , C > where S : Stream , C : Default + Extend < S :: Item > , { type Output = C ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < C > { let mut this = self . as_mut () . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (e) => this . collection . extend (Some (e)) , None => return Poll :: Ready (mem :: take (self . project () . collection)) , } } } }
    };
}

impl_99!()