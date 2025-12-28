macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl IsEnabled for i32 { }
    };
}

impl_17!();