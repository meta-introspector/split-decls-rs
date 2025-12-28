macro_rules! deps {
    () => {
        AsSymbolName!();
    };
}

macro_rules! impl_26 {
    () => {
        deps!();
        impl AsSymbolName for & [u8] { }
    };
}

impl_26!();