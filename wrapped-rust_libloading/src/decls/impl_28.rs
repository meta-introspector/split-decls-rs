macro_rules! deps {
    () => {
        AsSymbolName!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < const N : usize > AsSymbolName for [u8 ; N] { }
    };
}

impl_28!();