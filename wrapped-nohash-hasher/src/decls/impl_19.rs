macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl IsEnabled for isize { }
    };
}

impl_19!()