macro_rules! deps {
    () => {
        ObjectIdentifier!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl FromStr for ObjectIdentifier { type Err = Error ; fn from_str (string : & str) -> Result < Self > { Self :: new (string) } }
    };
}

impl_16!()