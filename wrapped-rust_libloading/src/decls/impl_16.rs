macro_rules! deps {
    () => {
        AsSymbolName!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl AsSymbolName for & String { }
    };
}

impl_16!();