macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! macro_75 {
    () => {
        deps!();
        __impl_public_bitflags_ops ! { Flags }
    };
}

macro_75!()