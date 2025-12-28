macro_rules! deps {
    () => {
        Flags!();
    };
}

macro_rules! macro_74 {
    () => {
        deps!();
        __impl_public_bitflags_forward ! { Flags : u32 , Field0 }
    };
}

macro_74!()