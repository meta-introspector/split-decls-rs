macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_609 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Fut , F , Item > Sink < Item > for OrElse < S , Fut , F > where S : Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_609!()