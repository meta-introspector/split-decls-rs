macro_rules! deps {
    () => {
        DecodePaddingMode!();
        EngineWrapper!();
    };
}

macro_rules! decode_pad_mode_requires_canonical_accepts_canonical {
    () => {
        deps!();
        # [doc = " Requires canonical padding -> accepts 2 + 2, 3 + 1, 4 + 0 final quad configurations"] # [apply (all_engines)] fn decode_pad_mode_requires_canonical_accepts_canonical < E : EngineWrapper > (engine_wrapper : E) { assert_all_suffixes_ok (E :: standard_with_pad_mode (true , DecodePaddingMode :: RequireCanonical) , vec ! ["/w==" , "iYU=" , "AAAA"] ,) ; }
    };
}

decode_pad_mode_requires_canonical_accepts_canonical!();