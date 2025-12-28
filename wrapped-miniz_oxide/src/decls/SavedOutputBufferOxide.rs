macro_rules! SavedOutputBufferOxide {
    () => {
        struct SavedOutputBufferOxide { pub pos : usize , pub bit_buffer : u32 , pub bits_in : u32 , pub local : bool , }
    };
}

SavedOutputBufferOxide!();