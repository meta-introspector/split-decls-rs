macro_rules! deps {
    () => {
        SplitStream!();
    };
}

macro_rules! impl_557 {
    () => {
        deps!();
        impl < S : Stream > Stream for SplitStream < S > { type Item = S :: Item ; fn poll_next (self : Pin < & mut Self > , cx : & mut Context < '_ >) -> Poll < Option < S :: Item > > { ready ! (self . 0 . poll_lock (cx)) . as_pin_mut () . poll_next (cx) } }
    };
}

impl_557!()