macro_rules! deps {
    () => {
        ClassLayout!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl ClassLayout < '_ > { pub fn packing_size (& self) -> u16 { self . usize (0) . try_into () . unwrap () } pub fn class_size (& self) -> u32 { self . usize (1) . try_into () . unwrap () } }
    };
}

impl_84!();