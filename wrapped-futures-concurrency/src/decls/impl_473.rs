macro_rules! deps {
    () => {
        WaitUntil!();
        State!();
    };
}

macro_rules! impl_473 {
    () => {
        deps!();
        impl < S , D > Stream for WaitUntil < S , D > where S : Stream , D : Future , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match this . state { State :: Timer => match this . deadline . poll (cx) { Poll :: Pending => Poll :: Pending , Poll :: Ready (_) => { * this . state = State :: Streaming ; this . stream . poll_next (cx) } } , State :: Streaming => this . stream . poll_next (cx) , } } }
    };
}

impl_473!();