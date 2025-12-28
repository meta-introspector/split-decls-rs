macro_rules! deps {
    () => {
        EngineWrapper!();
        DecodePaddingMode!();
    };
}

macro_rules! decode_pad_mode_indifferent_padding_accepts_anything {
    () => {
        deps!();
        # [doc = " Indifferent padding accepts 2 + 0-2, 3 + 0-1, 4 + 0 final chunk configuration"] # [apply (all_engines)] fn decode_pad_mode_indifferent_padding_accepts_anything < E : EngineWrapper > (engine_wrapper : E) { assert_all_suffixes_ok (E :: standard_with_pad_mode (true , DecodePaddingMode :: Indifferent) , vec ! ["/w" , "/w=" , "/w==" , "iYU" , "iYU=" , "AAAA"] ,) ; }
    };
}

decode_pad_mode_indifferent_padding_accepts_anything!();