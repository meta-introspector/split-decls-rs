macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl IsEnabled for u32 { }
    };
}

impl_12!()