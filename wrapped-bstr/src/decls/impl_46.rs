macro_rules! deps {
    () => {
        EscapeState!();
        EscapeBytes!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl < 'a > EscapeBytes < 'a > { pub (crate) fn new (bytes : & 'a [u8]) -> EscapeBytes < 'a > { EscapeBytes { remaining : bytes , state : EscapeState :: Start } } }
    };
}

impl_46!();