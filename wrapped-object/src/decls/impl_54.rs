macro_rules! deps {
    () => {
        Endian!();
        I32Bytes!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < E : Endian > I32Bytes < E > { # [doc = " Construct a new value given bytes that already have the required endianness."] pub const fn from_bytes (n : [u8 ; 4]) -> Self { Self (n , PhantomData) } # [doc = " Construct a new value given a native endian value."] pub fn new (e : E , n : i32) -> Self { Self (e . write_i32_bytes (n) , PhantomData) } # [doc = " Return the value as a native endian value."] pub fn get (self , e : E) -> i32 { e . read_i32_bytes (self . 0) } # [doc = " Set the value given a native endian value."] pub fn set (& mut self , e : E , n : i32) { self . 0 = e . write_i32_bytes (n) ; } }
    };
}

impl_54!()