macro_rules! deps {
    () => {
        SliceReader!();
    };
}

macro_rules! impl_396 {
    () => {
        deps!();
        impl < 'storage > SliceReader < 'storage > { # [doc = " Constructs a slice reader"] pub const fn new (bytes : & 'storage [u8]) -> SliceReader < 'storage > { SliceReader { slice : bytes } } }
    };
}

impl_396!()