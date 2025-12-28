macro_rules! impl_135 {
    () => {
        impl < S , P > Stream for TakeWhile < S , P > where S : Stream , P : FnMut (& S :: Item) -> bool , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match ready ! (this . stream . poll_next (cx)) { Some (v) => { if (this . predicate) (& v) { Poll :: Ready (Some (v)) } else { Poll :: Ready (None) } } None => Poll :: Ready (None) , } } }
    };
}

impl_135!()