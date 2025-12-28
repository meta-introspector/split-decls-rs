macro_rules! impl_101 {
    () => {
        impl < T , E , S , C > Future for TryCollectFuture < S , C > where S : Stream < Item = Result < T , E > > , C : Default + Extend < T > , { type Output = Result < C , E > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; Poll :: Ready (Ok (loop { match ready ! (this . stream . as_mut () . poll_next (cx) ?) { Some (x) => this . items . extend (Some (x)) , None => break mem :: take (this . items) , } })) } }
    };
}

impl_101!();