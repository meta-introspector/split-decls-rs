macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! impl_87 {
    () => {
        deps!();
        impl < S : Stream + ? Sized > StreamExt for S { }
    };
}

impl_87!()