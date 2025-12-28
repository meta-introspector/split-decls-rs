macro_rules! deps {
    () => {
        Deserializer!();
        SliceRead!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl < 'a > Deserializer < read :: SliceRead < 'a > > { # [doc = " Creates a JSON deserializer from a `&[u8]`."] pub fn from_slice (bytes : & 'a [u8]) -> Self { Deserializer :: new (read :: SliceRead :: new (bytes)) } }
    };
}

impl_15!()