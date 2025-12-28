macro_rules! LZOxide {
    () => {
        pub (crate) struct LZOxide { pub codes : [u8 ; LZ_CODE_BUF_SIZE] , pub code_position : usize , pub flag_position : usize , pub total_bytes : u32 , pub num_flags_left : u32 , }
    };
}

LZOxide!()