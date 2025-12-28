macro_rules! deps {
    () => {
        TryNextFuture!();
    };
}

macro_rules! impl_95 {
    () => {
        deps!();
        impl < T , E , S > Future for TryNextFuture < '_ , S > where S : Stream < Item = Result < T , E > > + Unpin + ? Sized , { type Output = Result < Option < T > , E > ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let res = ready ! (self . stream . poll_next (cx)) ; Poll :: Ready (res . transpose ()) } }
    };
}

impl_95!()