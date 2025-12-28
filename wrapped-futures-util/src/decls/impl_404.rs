macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_404 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Item > Sink < Item > for Peekable < S > where S : Sink < Item > + Stream , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_404!()