macro_rules! deps {
    () => {
        Error!();
        Result!();
        Value!();
    };
}

macro_rules! impl_249 {
    () => {
        deps!();
        impl FromStr for Value { type Err = Error ; fn from_str (s : & str) -> Result < Value , Error > { crate :: from_str (s) } }
    };
}

impl_249!();