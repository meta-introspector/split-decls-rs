macro_rules! deps {
    () => {
        Arg!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl From < & '_ Arg > for Arg { fn from (a : & Arg) -> Self { a . clone () } }
    };
}

impl_50!();