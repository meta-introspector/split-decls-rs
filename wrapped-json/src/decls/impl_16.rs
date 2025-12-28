macro_rules! deps {
    () => {
        Deserializer!();
        StrRead!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < 'a > Deserializer < read :: StrRead < 'a > > { # [doc = " Creates a JSON deserializer from a `&str`."] pub fn from_str (s : & 'a str) -> Self { Deserializer :: new (read :: StrRead :: new (s)) } }
    };
}

impl_16!()