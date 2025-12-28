macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_336 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Flatten < S , S :: Item > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_336!();