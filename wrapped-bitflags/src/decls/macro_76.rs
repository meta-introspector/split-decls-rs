macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! macro_76 {
    () => {
        deps!();
        __impl_public_bitflags_iter ! { Flags : u32 , Flags }
    };
}

macro_76!();