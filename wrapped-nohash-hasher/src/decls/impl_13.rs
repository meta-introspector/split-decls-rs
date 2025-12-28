macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl IsEnabled for u64 { }
    };
}

impl_13!();