macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_662 {
    () => {
        deps!();
        impl < St , C > Future for TryCollect < St , C > where St : TryStream , C : Default + Extend < St :: Ok > , { type Output = Result < C , St :: Error > ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; Poll :: Ready (Ok (loop { match ready ! (this . stream . as_mut () . try_poll_next (cx) ?) { Some (x) => this . items . extend (Some (x)) , None => break mem :: take (this . items) , } })) } }
    };
}

impl_662!()