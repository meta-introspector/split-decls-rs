macro_rules! impl_103 {
    () => {
        impl < S , P , B > Future for PartitionFuture < S , P , B > where S : Stream + Sized , P : FnMut (& S :: Item) -> bool , B : Default + Extend < S :: Item > , { type Output = (B , B) ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let mut this = self . project () ; loop { match ready ! (this . stream . as_mut () . poll_next (cx)) { Some (v) => { let res = this . res . as_mut () . unwrap () ; if (this . predicate) (& v) { res . 0 . extend (Some (v)) } else { res . 1 . extend (Some (v)) } } None => return Poll :: Ready (this . res . take () . unwrap ()) , } } } }
    };
}

impl_103!();