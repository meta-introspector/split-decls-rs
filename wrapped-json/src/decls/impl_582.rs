macro_rules! deps {
    () => {
        SliceRead!();
        StrRead!();
    };
}

macro_rules! impl_582 {
    () => {
        deps!();
        impl < 'a > StrRead < 'a > { # [doc = " Create a JSON input source to read from a UTF-8 string."] pub fn new (s : & 'a str) -> Self { StrRead { delegate : SliceRead :: new (s . as_bytes ()) , # [cfg (feature = "raw_value")] data : s , } } }
    };
}

impl_582!();