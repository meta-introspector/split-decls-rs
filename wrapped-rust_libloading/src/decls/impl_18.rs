macro_rules! deps {
    () => {
        AsSymbolName!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl AsSymbolName for String { }
    };
}

impl_18!();