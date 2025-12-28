macro_rules! deps {
    () => {
        AppSettings!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl AppSettings { fn bit (self) -> u32 { 1 << (self as u8) } }
    };
}

impl_41!()