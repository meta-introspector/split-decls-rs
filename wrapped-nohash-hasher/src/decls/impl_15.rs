macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl IsEnabled for i8 { }
    };
}

impl_15!()