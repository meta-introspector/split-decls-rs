macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl From < Error > for String { fn from (e : Error) -> String { e . message } }
    };
}

impl_483!();