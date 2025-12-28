macro_rules! deps {
    () => {
        DecodePaddingMode!();
        EngineWrapper!();
    };
}

macro_rules! decode_pad_mode_requires_no_padding_accepts_no_padding {
    () => {
        deps!();
        # [doc = " Requires no padding -> accepts 2 + 0, 3 + 0, 4 + 0 final chunk configuration"] # [apply (all_engines)] fn decode_pad_mode_requires_no_padding_accepts_no_padding < E : EngineWrapper > (engine_wrapper : E) { assert_all_suffixes_ok (E :: standard_with_pad_mode (true , DecodePaddingMode :: RequireNone) , vec ! ["/w" , "iYU" , "AAAA"] ,) ; }
    };
}

decode_pad_mode_requires_no_padding_accepts_no_padding!()