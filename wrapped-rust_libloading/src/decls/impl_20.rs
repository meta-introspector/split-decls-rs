macro_rules! deps {
    () => {
        AsSymbolName!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl AsSymbolName for & CStr { }
    };
}

impl_20!()