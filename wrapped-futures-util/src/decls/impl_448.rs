macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for TakeWhile < S , Fut , F > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_448!();