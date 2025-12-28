macro_rules! impl_105 {
    () => {
        impl < S , F , T > Future for FoldFuture < S , F , T > where S : Stream , F : FnMut (T , S :: Item) -> T , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (v) => { let old = this . acc . take () . unwrap () ; let new = (this . f) (old , v) ; * this . acc = Some (new) ; } None => return Poll :: Ready (this . acc . take () . unwrap ()) , } } } }
    };
}

impl_105!()