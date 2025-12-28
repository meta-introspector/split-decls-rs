macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_373 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S : Stream + Sink < Item > , Item > Sink < Item > for Fuse < S > { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_373!();