macro_rules! deps {
    () => {
        SplitStream!();
        SplitSink!();
    };
}

macro_rules! impl_555 {
    () => {
        deps!();
        impl < S > SplitStream < S > { # [doc = " Returns `true` if the `SplitStream<S>` and `SplitSink<S>` originate from the same call to `StreamExt::split`."] pub fn is_pair_of < Item > (& self , other : & SplitSink < S , Item >) -> bool { other . is_pair_of (& self) } }
    };
}

impl_555!()