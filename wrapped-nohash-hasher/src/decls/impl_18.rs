macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl IsEnabled for i64 { }
    };
}

impl_18!();