macro_rules! impl_184 {
    () => {
        impl < S , A , B , FromA , FromB > Future for UnzipFuture < S , FromA , FromB > where S : Stream < Item = (A , B) > , FromA : Default + Extend < A > , FromB : Default + Extend < B > , { type Output = (FromA , FromB) ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some ((a , b)) => { let res = this . res . as_mut () . unwrap () ; res . 0 . extend (Some (a)) ; res . 1 . extend (Some (b)) ; } None => return Poll :: Ready (this . res . take () . unwrap ()) , } } } }
    };
}

impl_184!();