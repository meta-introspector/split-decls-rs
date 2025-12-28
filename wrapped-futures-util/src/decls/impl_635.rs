macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_635 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for TryFilterMap < S , Fut , F > where S : Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_635!()