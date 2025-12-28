macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_490 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Item > Sink < Item > for ReadyChunks < S > where S : Stream + Sink < Item > , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_490!();