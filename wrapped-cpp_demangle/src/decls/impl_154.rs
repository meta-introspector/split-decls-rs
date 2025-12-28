macro_rules! deps {
    () => {
        CtorDtorName!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl CtorDtorName { # [inline] fn starts_with (byte : u8) -> bool { byte == b'C' || byte == b'D' } }
    };
}

impl_154!();