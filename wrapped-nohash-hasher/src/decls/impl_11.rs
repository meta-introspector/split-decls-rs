macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl IsEnabled for u16 { }
    };
}

impl_11!()