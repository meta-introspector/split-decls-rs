macro_rules! deps {
    () => {
        StreamExt!();
    };
}

macro_rules! impl_583 {
    () => {
        deps!();
        impl < T : ? Sized > StreamExt for T where T : Stream { }
    };
}

impl_583!()