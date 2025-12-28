macro_rules! deps {
    () => {
        Ready!();
        StreamFuture!();
    };
}

macro_rules! impl_378 {
    () => {
        deps!();
        impl < St : Stream + Unpin > Future for StreamFuture < St > { type Output = (Option < St :: Item > , St) ; fn poll (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let item = { let s = self . stream . as_mut () . expect ("polling StreamFuture twice") ; ready ! (s . poll_next_unpin (cx)) } ; let stream = self . stream . take () . unwrap () ; Poll :: Ready ((item , stream)) } }
    };
}

impl_378!();