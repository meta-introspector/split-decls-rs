macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_315 {
    () => {
        deps!();
        impl < St : Stream > Stream for Enumerate < St > { type Item = (usize , St :: Item) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match ready ! (this . stream . poll_next (cx)) { Some (item) => { let prev_count = * this . count ; * this . count += 1 ; Poll :: Ready (Some ((prev_count , item))) } None => Poll :: Ready (None) , } } fn size_hint (& self) -> (usize , Option < usize >) { self . stream . size_hint () } }
    };
}

impl_315!();