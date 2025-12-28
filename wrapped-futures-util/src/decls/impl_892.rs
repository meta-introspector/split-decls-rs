macro_rules! deps {
    () => {
        SelectAll!();
        Ready!();
    };
}

macro_rules! impl_892 {
    () => {
        deps!();
        impl < St : Stream + Unpin > Stream for SelectAll < St > { type Item = St :: Item ; fn poll_next (mut self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { loop { match ready ! (self . inner . poll_next_unpin (cx)) { Some ((Some (item) , remaining)) => { self . push (remaining) ; return Poll :: Ready (Some (item)) ; } Some ((None , _)) => { } None => return Poll :: Ready (None) , } } } }
    };
}

impl_892!()