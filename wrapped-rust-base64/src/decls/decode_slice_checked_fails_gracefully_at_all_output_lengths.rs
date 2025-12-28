macro_rules! deps {
    () => {
        DecodePaddingMode!();
        EngineWrapper!();
        DecodeSliceError!();
    };
}

macro_rules! decode_slice_checked_fails_gracefully_at_all_output_lengths {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_slice_checked_fails_gracefully_at_all_output_lengths < E : EngineWrapper > (engine_wrapper : E ,) { let mut rng = seeded_rng () ; for original_len in 0 .. 1000 { let mut original = vec ! [0 ; original_len] ; rng . fill (& mut original [..]) ; for mode in all_pad_modes () { let engine = E :: standard_with_pad_mode (match mode { DecodePaddingMode :: Indifferent | DecodePaddingMode :: RequireCanonical => true , DecodePaddingMode :: RequireNone => false , } , mode ,) ; let encoded = engine . encode (& original) ; let mut decode_buf = Vec :: with_capacity (original_len) ; for decode_buf_len in 0 .. original_len { decode_buf . resize (decode_buf_len , 0) ; assert_eq ! (DecodeSliceError :: OutputSliceTooSmall , engine . decode_slice (& encoded , & mut decode_buf [..]) . unwrap_err () , "original len: {}, encoded len: {}, buf len: {}, mode: {:?}" , original_len , encoded . len () , decode_buf_len , mode) ; assert_eq ! (DecodeSliceError :: OutputSliceTooSmall , engine . internal_decode (encoded . as_bytes () , & mut decode_buf [..] , engine . internal_decoded_len_estimate (encoded . len ())) . unwrap_err ()) ; } decode_buf . resize (original_len , 0) ; rng . fill (& mut decode_buf [..]) ; assert_eq ! (original_len , engine . decode_slice (& encoded , & mut decode_buf [..]) . unwrap ()) ; assert_eq ! (original , decode_buf) ; } } }
    };
}

decode_slice_checked_fails_gracefully_at_all_output_lengths!();