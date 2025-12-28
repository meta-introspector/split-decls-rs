macro_rules! deps {
    () => {
        IntoConcurrentStream!();
        ConcurrentStream!();
    };
}

macro_rules! impl_161 {
    () => {
        deps!();
        impl < S : ConcurrentStream > IntoConcurrentStream for S { type Item = S :: Item ; type IntoConcurrentStream = S ; fn into_co_stream (self) -> Self :: IntoConcurrentStream { self } }
    };
}

impl_161!()