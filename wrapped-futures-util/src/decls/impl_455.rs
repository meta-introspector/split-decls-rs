macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Fut , Item > Sink < Item > for TakeUntil < S , Fut > where S : Stream + Sink < Item > , Fut : Future , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_455!();