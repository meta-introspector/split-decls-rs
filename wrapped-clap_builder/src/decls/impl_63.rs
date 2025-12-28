macro_rules! deps {
    () => {
        ArgGroup!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl From < & '_ ArgGroup > for ArgGroup { fn from (g : & ArgGroup) -> Self { g . clone () } }
    };
}

impl_63!();