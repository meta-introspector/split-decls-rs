macro_rules! deps {
    () => {
        Context!();
        Timer!();
    };
}

macro_rules! impl_648 {
    () => {
        deps!();
        impl Stream for Timer { type Item = () ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = & mut * self ; match this . delay . poll_unpin (cx) { Poll :: Ready (_) => { this . delay . reset (this . interval) ; Poll :: Ready (Some (())) } Poll :: Pending => Poll :: Pending , } } }
    };
}

impl_648!();