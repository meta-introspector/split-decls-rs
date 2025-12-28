macro_rules! deps {
    () => {
        Ty!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl From < Box < Ty > > for Ty { fn from (value : Box < Ty >) -> Self { * value } }
    };
}

impl_131!();