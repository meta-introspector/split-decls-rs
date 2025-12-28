macro_rules! deps {
    () => {
        RotateEachWord64!();
        BitOps32!();
    };
}

macro_rules! BitOps64 {
    () => {
        deps!();
        pub trait BitOps64 : BitOps32 + RotateEachWord64 { }
    };
}

BitOps64!()