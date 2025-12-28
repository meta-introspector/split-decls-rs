macro_rules! deps {
    () => {
        SplitSink!();
    };
}

macro_rules! impl_560 {
    () => {
        deps!();
        impl < S , Item > Unpin for SplitSink < S , Item > { }
    };
}

impl_560!()