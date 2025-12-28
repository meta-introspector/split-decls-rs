macro_rules! deps {
    () => {
        Lzma2Writer!();
        Result!();
        Write!();
    };
}

macro_rules! impl_214 {
    () => {
        deps!();
        impl < W : Write > Write for Lzma2Writer < W > { fn write (& mut self , buf : & [u8]) -> crate :: Result < usize > { let mut len = buf . len () ; let mut off = 0 ; while len > 0 { if self . should_start_independent_chunk () { self . start_independent_chunk () ? ; } let used = self . lzma . lz . fill_window (& buf [off .. (off + len)]) ; off += used ; len -= used ; self . pending_size += used as u32 ; if self . lzma . encode_for_lzma2 (& mut self . rc , & mut self . mode) ? { self . write_chunk () ? ; } } Ok (off) } fn flush (& mut self) -> crate :: Result < () > { self . lzma . lz . set_flushing () ; while self . pending_size > 0 { self . lzma . encode_for_lzma2 (& mut self . rc , & mut self . mode) ? ; self . write_chunk () ? ; } self . inner . flush () } }
    };
}

impl_214!();