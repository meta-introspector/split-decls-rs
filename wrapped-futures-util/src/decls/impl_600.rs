macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_600 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S : Sink < Item > , Item > Sink < Item > for IntoStream < S > { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_600!()