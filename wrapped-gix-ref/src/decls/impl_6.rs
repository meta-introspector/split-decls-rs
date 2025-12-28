macro_rules! deps {
    () => {
        FullName!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl From < FullName > for BString { fn from (name : FullName) -> Self { name . 0 } }
    };
}

impl_6!()