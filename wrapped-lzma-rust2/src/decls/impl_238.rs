macro_rules! deps {
    () => {
        Result!();
        RangeEncoderBuffer!();
        Write!();
    };
}

macro_rules! impl_238 {
    () => {
        deps!();
        impl Write for RangeEncoderBuffer { fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > { let size = buf . len () . min (self . buf . len () - self . pos) ; if size == 0 { return Ok (0) ; } self . buf [self . pos .. (self . pos + size)] . copy_from_slice (& buf [.. size]) ; self . pos += size ; Ok (size) } fn flush (& mut self) -> crate :: Result < () > { Ok (()) } }
    };
}

impl_238!();