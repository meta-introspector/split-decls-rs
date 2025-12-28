macro_rules! impl_137 {
    () => {
        impl < B , S , P > Stream for MapWhile < S , P > where S : Stream , P : FnMut (S :: Item) -> Option < B > , { type Item = B ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match ready ! (this . stream . poll_next (cx)) { Some (v) => Poll :: Ready ((this . predicate) (v)) , None => Poll :: Ready (None) , } } }
    };
}

impl_137!();