macro_rules! deps {
    () => {
        Endian!();
        I16Bytes!();
    };
}

macro_rules! impl_52 {
    () => {
        deps!();
        impl < E : Endian > I16Bytes < E > { # [doc = " Construct a new value given bytes that already have the required endianness."] pub const fn from_bytes (n : [u8 ; 2]) -> Self { Self (n , PhantomData) } # [doc = " Construct a new value given a native endian value."] pub fn new (e : E , n : i16) -> Self { Self (e . write_i16_bytes (n) , PhantomData) } # [doc = " Return the value as a native endian value."] pub fn get (self , e : E) -> i16 { e . read_i16_bytes (self . 0) } # [doc = " Set the value given a native endian value."] pub fn set (& mut self , e : E , n : i16) { self . 0 = e . write_i16_bytes (n) ; } }
    };
}

impl_52!();