macro_rules! deps {
    () => {
        ArgSettings!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl ArgSettings { fn bit (self) -> u32 { 1 << (self as u8) } }
    };
}

impl_73!()