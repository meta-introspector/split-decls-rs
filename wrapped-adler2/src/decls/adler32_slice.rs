macro_rules! deps {
    () => {
        Adler32!();
    };
}

macro_rules! adler32_slice {
    () => {
        deps!();
        # [doc = " Calculates the Adler-32 checksum of a byte slice."] # [doc = ""] # [doc = " This is a convenience function around the [`Adler32`] type."] # [doc = ""] # [doc = " [`Adler32`]: struct.Adler32.html"] pub fn adler32_slice (data : & [u8]) -> u32 { let mut h = Adler32 :: new () ; h . write_slice (data) ; h . checksum () }
    };
}

adler32_slice!()