macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_684 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Item > Sink < Item > for TryReadyChunks < S > where S : TryStream + Sink < Item > , { type Error = < S as Sink < Item > > :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_684!();