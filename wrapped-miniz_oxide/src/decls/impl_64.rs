macro_rules! deps {
    () => {
        Error!();
        BitBuffer!();
        OutputBufferOxide!();
        Result!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl BitBuffer { fn put_fast (& mut self , bits : u64 , len : u32) { self . bit_buffer |= bits << self . bits_in ; self . bits_in += len ; } fn flush (& mut self , output : & mut OutputBufferOxide) -> Result < () > { let pos = output . inner_pos ; { let inner = & mut output . inner [pos .. pos + 8] ; let bytes = u64 :: to_le_bytes (self . bit_buffer) ; inner . copy_from_slice (& bytes) ; } match output . inner_pos . checked_add ((self . bits_in >> 3) as usize) { Some (n) if n <= output . inner . len () => output . inner_pos = n , _ => return Err (Error { }) , } self . bit_buffer >>= self . bits_in & ! 7 ; self . bits_in &= 7 ; Ok (()) } }
    };
}

impl_64!();