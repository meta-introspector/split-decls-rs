macro_rules! deps {
    () => {
        CompressorOxide!();
        TDEFLFlush!();
        TDEFLStatus!();
    };
}

macro_rules! compress_to_vec_inner {
    () => {
        deps!();
        # [doc = " Simple function to compress data to a vec."] fn compress_to_vec_inner (mut input : & [u8] , level : u8 , window_bits : i32 , strategy : i32) -> Vec < u8 > { let flags = create_comp_flags_from_zip_params (level . into () , window_bits , strategy) ; let mut compressor = CompressorOxide :: new (flags) ; let mut output = vec ! [0 ; :: core :: cmp :: max (input . len () / 2 , 2)] ; let mut out_pos = 0 ; loop { let (status , bytes_in , bytes_out) = compress (& mut compressor , input , & mut output [out_pos ..] , TDEFLFlush :: Finish ,) ; out_pos += bytes_out ; match status { TDEFLStatus :: Done => { output . truncate (out_pos) ; break ; } TDEFLStatus :: Okay if bytes_in <= input . len () => { input = & input [bytes_in ..] ; if output . len () . saturating_sub (out_pos) < 30 { output . resize (output . len () * 2 , 0) } } _ => panic ! ("Bug! Unexpectedly failed to compress!") , } } output }
    };
}

compress_to_vec_inner!();