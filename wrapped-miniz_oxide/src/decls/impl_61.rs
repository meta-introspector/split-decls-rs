macro_rules! deps {
    () => {
        OutputBufferOxide!();
        SavedOutputBufferOxide!();
    };
}

macro_rules! impl_61 {
    () => {
        deps!();
        impl OutputBufferOxide < '_ > { # [doc = " Write bits to the bit buffer and flushes"] # [doc = " the bit buffer so any whole bytes are output"] # [doc = " to the underlying buffer."] fn put_bits (& mut self , bits : u32 , len : u32) { assert ! (bits <= ((1u32 << len) - 1u32)) ; self . bit_buffer |= bits << self . bits_in ; self . bits_in += len ; while self . bits_in >= 8 { self . inner [self . inner_pos] = self . bit_buffer as u8 ; self . inner_pos += 1 ; self . bit_buffer >>= 8 ; self . bits_in -= 8 ; } } # [inline] # [doc = " Write the provided bits to the bit buffer without flushing"] # [doc = " anything. Does not check if there is actually space for it."] fn put_bits_no_flush (& mut self , bits : u32 , len : u32) { self . bit_buffer |= bits << self . bits_in ; self . bits_in += len ; } const fn save (& self) -> SavedOutputBufferOxide { SavedOutputBufferOxide { pos : self . inner_pos , bit_buffer : self . bit_buffer , bits_in : self . bits_in , local : self . local , } } fn load (& mut self , saved : SavedOutputBufferOxide) { self . inner_pos = saved . pos ; self . bit_buffer = saved . bit_buffer ; self . bits_in = saved . bits_in ; self . local = saved . local ; } # [inline] # [doc = " Pad the bit buffer to a whole byte with"] # [doc = " zeroes and write that byte to the output buffer."] fn pad_to_bytes (& mut self) { if self . bits_in != 0 { let len = 8 - self . bits_in ; self . put_bits (0 , len) ; } } # [inline] fn write_bytes (& mut self , bytes : & [u8]) { debug_assert_eq ! (self . bits_in , 0) ; self . inner [self . inner_pos .. self . inner_pos + bytes . len ()] . copy_from_slice (bytes) ; self . inner_pos += bytes . len () ; } }
    };
}

impl_61!()