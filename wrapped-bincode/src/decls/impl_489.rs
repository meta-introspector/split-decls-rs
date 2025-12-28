macro_rules! deps {
    () => {
        SliceWriter!();
    };
}

macro_rules! impl_489 {
    () => {
        deps!();
        impl < 'storage > SliceWriter < 'storage > { # [doc = " Create a new instance of `SliceWriter` with the given byte array."] pub fn new (bytes : & 'storage mut [u8]) -> SliceWriter < 'storage > { let original = bytes . len () ; SliceWriter { slice : bytes , original_length : original , } } # [doc = " Return the amount of bytes written so far."] pub fn bytes_written (& self) -> usize { self . original_length - self . slice . len () } }
    };
}

impl_489!()