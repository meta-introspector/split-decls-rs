macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl From < Error > for String { fn from (e : Error) -> String { e . message } }
    };
}

impl_27!()