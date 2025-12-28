macro_rules! impl_147 {
    () => {
        impl < 'a , S , T : 'a > Stream for Cloned < S > where S : Stream < Item = & 'a T > , T : Clone , { type Item = T ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let this = self . project () ; let next = ready ! (this . stream . poll_next (cx)) ; Poll :: Ready (next . cloned ()) } }
    };
}

impl_147!();