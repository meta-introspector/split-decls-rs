macro_rules! deps {
    () => {
        Fields!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        # [cfg (feature = "unicode")] impl < 'a > Fields < 'a > { fn new (bytes : & 'a [u8]) -> Fields < 'a > { Fields { it : bytes . fields_with (char :: is_whitespace) } } }
    };
}

impl_83!();