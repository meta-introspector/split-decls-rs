macro_rules! impl_125 {
    () => {
        impl < T , S1 , S2 > Stream for Or < S1 , S2 > where S1 : Stream < Item = T > , S2 : Stream < Item = T > , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; if let Poll :: Ready (Some (t)) = this . stream1 . as_mut () . poll_next (cx) { return Poll :: Ready (Some (t)) ; } this . stream2 . as_mut () . poll_next (cx) } }
    };
}

impl_125!();