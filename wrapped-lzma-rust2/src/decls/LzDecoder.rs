macro_rules! LzDecoder {
    () => {
        # [derive (Default)] pub (crate) struct LzDecoder { buf : Vec < u8 > , buf_size : usize , start : usize , pos : usize , full : usize , limit : usize , pending_len : usize , pending_dist : usize , }
    };
}

LzDecoder!()