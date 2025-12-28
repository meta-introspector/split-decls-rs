macro_rules! deps {
    () => {
        Ready!();
    };
}

macro_rules! impl_403 {
    () => {
        deps!();
        impl < S : Stream > Stream for Peekable < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; if let Some (item) = this . peeked . take () { return Poll :: Ready (Some (item)) ; } this . stream . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { let peek_len = usize :: from (self . peeked . is_some ()) ; let (lower , upper) = self . stream . size_hint () ; let lower = lower . saturating_add (peek_len) ; let upper = match upper { Some (x) => x . checked_add (peek_len) , None => None , } ; (lower , upper) } }
    };
}

impl_403!();