macro_rules! deps {
    () => {
        Result!();
        Write!();
        LzmaWriter!();
    };
}

macro_rules! impl_226 {
    () => {
        deps!();
        impl < W : Write > Write for LzmaWriter < W > { fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > { if let Some (exp) = self . expected_uncompressed_size { if exp < self . current_uncompressed_size + buf . len () as u64 { return Err (error_invalid_input ("expected compressed size does not match actual compressed size" ,)) ; } } self . current_uncompressed_size += buf . len () as u64 ; let mut len = buf . len () ; let mut off = 0 ; while len > 0 { let used = self . lzma . lz . fill_window (& buf [off ..]) ; off += used ; len -= used ; self . lzma . encode_for_lzma1 (& mut self . rc , & mut self . mode) ? ; } Ok (off) } fn flush (& mut self) -> crate :: Result < () > { Ok (()) } }
    };
}

impl_226!();