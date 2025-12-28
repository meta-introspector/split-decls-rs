macro_rules! deps {
    () => {
        Keyed!();
        Key!();
    };
}

macro_rules! impl_399 {
    () => {
        deps!();
        impl < S : Stream > Stream for Keyed < S > { type Item = (Key , < S as Stream > :: Item) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; this . group . as_mut () . poll_next_inner (cx) } }
    };
}

impl_399!();