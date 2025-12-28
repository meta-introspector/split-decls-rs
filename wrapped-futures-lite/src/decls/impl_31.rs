macro_rules! deps {
    () => {
        Pending!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        # [cfg (feature = "race")] impl < T , F1 , F2 > Future for Race < F1 , F2 > where F1 : Future < Output = T > , F2 : Future < Output = T > , { type Output = T ; fn poll (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Self :: Output > { let this = self . project () ; if this . rng . bool () { if let Poll :: Ready (t) = this . future1 . poll (cx) { return Poll :: Ready (t) ; } if let Poll :: Ready (t) = this . future2 . poll (cx) { return Poll :: Ready (t) ; } } else { if let Poll :: Ready (t) = this . future2 . poll (cx) { return Poll :: Ready (t) ; } if let Poll :: Ready (t) = this . future1 . poll (cx) { return Poll :: Ready (t) ; } } Poll :: Pending } }
    };
}

impl_31!();