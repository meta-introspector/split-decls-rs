macro_rules! deps {
    () => {
        BitOps0!();
        RotateEachWord32!();
    };
}

macro_rules! BitOps32 {
    () => {
        deps!();
        pub trait BitOps32 : BitOps0 + RotateEachWord32 { }
    };
}

BitOps32!();