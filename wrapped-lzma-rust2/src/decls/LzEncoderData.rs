macro_rules! LzEncoderData {
    () => {
        pub (crate) struct LzEncoderData { pub (crate) keep_size_before : u32 , pub (crate) keep_size_after : u32 , pub (crate) match_len_max : u32 , pub (crate) nice_len : u32 , pub (crate) buf : Vec < u8 > , pub (crate) buf_size : usize , pub (crate) buf_limit_u16 : usize , pub (crate) read_pos : i32 , pub (crate) read_limit : i32 , pub (crate) finishing : bool , pub (crate) write_pos : i32 , pub (crate) pending_size : u32 , }
    };
}

LzEncoderData!()