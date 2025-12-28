macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl IsEnabled for u8 { }
    };
}

impl_10!();