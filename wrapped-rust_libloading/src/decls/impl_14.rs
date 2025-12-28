macro_rules! deps {
    () => {
        AsSymbolName!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl AsSymbolName for & str { }
    };
}

impl_14!()