macro_rules! deps {
    () => {
        Map!();
        Result!();
        Value!();
        Error!();
    };
}

macro_rules! impl_250 {
    () => {
        deps!();
        impl FromStr for Map < String , Value > { type Err = Error ; fn from_str (s : & str) -> Result < Self , Error > { crate :: from_str (s) } }
    };
}

impl_250!()