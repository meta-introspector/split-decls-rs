macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl IsEnabled for i16 { }
    };
}

impl_16!()