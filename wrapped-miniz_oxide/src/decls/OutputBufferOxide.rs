macro_rules! OutputBufferOxide {
    () => {
        struct OutputBufferOxide < 'a > { pub inner : & 'a mut [u8] , pub inner_pos : usize , pub local : bool , pub bit_buffer : u32 , pub bits_in : u32 , }
    };
}

OutputBufferOxide!();