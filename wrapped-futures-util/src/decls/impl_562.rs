macro_rules! deps {
    () => {
        SplitStream!();
        SplitSink!();
    };
}

macro_rules! impl_562 {
    () => {
        deps!();
        impl < S , Item > SplitSink < S , Item > { # [doc = " Returns `true` if the `SplitStream<S>` and `SplitSink<S>` originate from the same call to `StreamExt::split`."] pub fn is_pair_of (& self , other : & SplitStream < S >) -> bool { self . lock . is_pair_of (& other . 0) } }
    };
}

impl_562!()