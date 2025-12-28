macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_984 {
    () => {
        deps!();
        impl < S , Item > Stream for Buffer < S , Item > where S : Sink < Item > + Stream , { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < S :: Item > > { self . project () . sink . poll_next (cx) } fn size_hint (& self) -> (usize , Option < usize >) { self . sink . size_hint () } }
    };
}

impl_984!()