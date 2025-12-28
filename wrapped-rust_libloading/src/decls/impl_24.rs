macro_rules! deps {
    () => {
        AsSymbolName!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl AsSymbolName for CString { }
    };
}

impl_24!()