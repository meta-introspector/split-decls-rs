macro_rules! deps {
    () => {
        BitOps64!();
        BitOps0!();
    };
}

macro_rules! impl_42 {
    () => {
        deps!();
        impl < W > BitOps64 for x4 < W > where W : BitOps64 + BitOps0 { }
    };
}

impl_42!()