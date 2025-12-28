macro_rules! deps {
    () => {
        StreamGroup!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl < S : Stream > Stream for StreamGroup < S > { type Item = < S as Stream > :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { match self . poll_next_inner (cx) { Poll :: Ready (Some ((_key , item))) => Poll :: Ready (Some (item)) , Poll :: Ready (None) => Poll :: Ready (None) , Poll :: Pending => Poll :: Pending , } } }
    };
}

impl_393!();