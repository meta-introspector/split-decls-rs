macro_rules! deps {
    () => {
        Sink!();
    };
}

macro_rules! impl_505 {
    () => {
        deps!();
        # [cfg (feature = "sink")] impl < S , Item > Sink < Item > for BufferUnordered < S > where S : Stream + Sink < Item > , S :: Item : Future , { type Error = S :: Error ; delegate_sink ! (stream , Item) ; }
    };
}

impl_505!()