macro_rules! deps {
    () => {
        IsEnabled!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl IsEnabled for usize { }
    };
}

impl_14!();