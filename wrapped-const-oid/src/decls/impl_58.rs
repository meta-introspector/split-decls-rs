macro_rules! deps {
    () => {
        ObjectIdentifier!();
        Result!();
        Error!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl FromStr for ObjectIdentifier { type Err = Error ; fn from_str (string : & str) -> Result < Self > { Self :: new (string) } }
    };
}

impl_58!()