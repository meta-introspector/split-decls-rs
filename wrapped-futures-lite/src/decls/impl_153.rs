macro_rules! impl_153 {
    () => {
        impl < S > Stream for Enumerate < S > where S : Stream , { type Item = (usize , S :: Item) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; match ready ! (this . stream . poll_next (cx)) { Some (v) => { let ret = (* this . i , v) ; * this . i += 1 ; Poll :: Ready (Some (ret)) } None => Poll :: Ready (None) , } } }
    };
}

impl_153!()