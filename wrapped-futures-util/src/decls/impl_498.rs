macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_498 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < St , S , Fut , F , Item > Sink < Item > for Scan < St , S , Fut , F > where St : Stream + Sink < Item > , { type Error = St :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_498!()