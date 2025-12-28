macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < T , F1 , F2 > Future for Or < F1 , F2 > where F1 : Future < Output = T > , F2 : Future < Output = T > , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if let Poll :: Ready (t) = this . future1 . poll (cx) { return Poll :: Ready (t) ; } if let Poll :: Ready (t) = this . future2 . poll (cx) { return Poll :: Ready (t) ; } Poll :: Pending } }
    };
}

impl_23!();