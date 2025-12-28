macro_rules! deps {
    () => {
        UnnamedTypeName!();
    };
}

macro_rules! impl_204 {
    () => {
        deps!();
        impl UnnamedTypeName { # [inline] fn starts_with (byte : u8) -> bool { byte == b'U' } }
    };
}

impl_204!()