macro_rules! deps {
    () => {
        U64Bytes!();
        Endian!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < E : Endian > U64Bytes < E > { # [doc = " Construct a new value given bytes that already have the required endianness."] pub const fn from_bytes (n : [u8 ; 8]) -> Self { Self (n , PhantomData) } # [doc = " Construct a new value given a native endian value."] pub fn new (e : E , n : u64) -> Self { Self (e . write_u64_bytes (n) , PhantomData) } # [doc = " Return the value as a native endian value."] pub fn get (self , e : E) -> u64 { e . read_u64_bytes (self . 0) } # [doc = " Set the value given a native endian value."] pub fn set (& mut self , e : E , n : u64) { self . 0 = e . write_u64_bytes (n) ; } }
    };
}

impl_50!()