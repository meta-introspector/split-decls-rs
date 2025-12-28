macro_rules! deps {
    () => {
        InflateState!();
        MZError!();
        MZStatus!();
        MZFlush!();
        TINFLStatus!();
        MZResult!();
    };
}

macro_rules! inflate_loop {
    () => {
        deps!();
        fn inflate_loop (state : & mut InflateState , next_in : & mut & [u8] , next_out : & mut & mut [u8] , total_in : & mut usize , total_out : & mut usize , decomp_flags : u32 , flush : MZFlush ,) -> MZResult { let orig_in_len = next_in . len () ; loop { let status = decompress (& mut state . decomp , next_in , & mut state . dict , state . dict_ofs , decomp_flags ,) ; let in_bytes = status . 1 ; let out_bytes = status . 2 ; let status = status . 0 ; state . last_status = status ; * next_in = & next_in [in_bytes ..] ; * total_in += in_bytes ; state . dict_avail = out_bytes ; * total_out += push_dict_out (state , next_out) ; if status == TINFLStatus :: FailedCannotMakeProgress { return Err (MZError :: Buf) ; } else if (status as i32) < 0 { return Err (MZError :: Data) ; } if (status == TINFLStatus :: NeedsMoreInput) && orig_in_len == 0 { return Err (MZError :: Buf) ; } if flush == MZFlush :: Finish { if status == TINFLStatus :: Done { return if state . dict_avail != 0 { Err (MZError :: Buf) } else { Ok (MZStatus :: StreamEnd) } ; } else if next_out . is_empty () { return Err (MZError :: Buf) ; } } else { let empty_buf = next_in . is_empty () || next_out . is_empty () ; if (status == TINFLStatus :: Done) || empty_buf || (state . dict_avail != 0) { return if (status == TINFLStatus :: Done) && (state . dict_avail == 0) { Ok (MZStatus :: StreamEnd) } else { Ok (MZStatus :: Ok) } ; } } } }
    };
}

inflate_loop!()