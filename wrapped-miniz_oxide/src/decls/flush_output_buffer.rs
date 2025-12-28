macro_rules! deps {
    () => {
        CallbackOut!();
        CallbackOxide!();
        TDEFLStatus!();
        ParamsOxide!();
    };
}

macro_rules! flush_output_buffer {
    () => {
        deps!();
        fn flush_output_buffer (c : & mut CallbackOxide , p : & mut ParamsOxide) -> (TDEFLStatus , usize , usize) { let mut res = (TDEFLStatus :: Okay , p . src_pos , 0) ; if let CallbackOut :: Buf (ref mut cb) = c . out { let n = cmp :: min (cb . out_buf . len () - p . out_buf_ofs , p . flush_remaining as usize) ; if n != 0 { cb . out_buf [p . out_buf_ofs .. p . out_buf_ofs + n] . copy_from_slice (& p . local_buf . b [p . flush_ofs as usize .. p . flush_ofs as usize + n]) ; } p . flush_ofs += n as u32 ; p . flush_remaining -= n as u32 ; p . out_buf_ofs += n ; res . 2 = p . out_buf_ofs ; } if p . finished && p . flush_remaining == 0 { res . 0 = TDEFLStatus :: Done } res }
    };
}

flush_output_buffer!()