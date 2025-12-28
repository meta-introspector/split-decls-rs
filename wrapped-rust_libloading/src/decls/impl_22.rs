macro_rules! deps {
    () => {
        AsSymbolName!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl AsSymbolName for & CString { }
    };
}

impl_22!();