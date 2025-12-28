macro_rules! deps {
    () => {
        BitOps64!();
        RotateEachWord128!();
    };
}

macro_rules! BitOps128 {
    () => {
        deps!();
        pub trait BitOps128 : BitOps64 + RotateEachWord128 { }
    };
}

BitOps128!();