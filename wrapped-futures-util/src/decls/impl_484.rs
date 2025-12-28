macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_484 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Chunks < S > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_484!()