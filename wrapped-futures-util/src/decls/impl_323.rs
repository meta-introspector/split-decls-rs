macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for Filter < S , Fut , F > where S : Stream + Sink < Item > , F : FnMut (& S :: Item) -> Fut , Fut : Future < Output = bool > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_323!()