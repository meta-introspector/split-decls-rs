macro_rules! deps {
    () => {
        Key!();
        Keyed!();
    };
}

macro_rules! impl_211 {
    () => {
        deps!();
        impl < F : Future > Stream for Keyed < F > { type Item = (Key , < F as Future > :: Output) ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < Self :: Item > > { let mut this = self . project () ; this . group . as_mut () . poll_next_inner (cx) } }
    };
}

impl_211!();