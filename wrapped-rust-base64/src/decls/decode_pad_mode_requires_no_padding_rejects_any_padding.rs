macro_rules! deps {
    () => {
        EngineWrapper!();
        DecodeError!();
        DecodePaddingMode!();
    };
}

macro_rules! decode_pad_mode_requires_no_padding_rejects_any_padding {
    () => {
        deps!();
        # [doc = " Requires no padding -> rejects 2 + 1-2, 3 + 1 final chunk configuration"] # [apply (all_engines)] fn decode_pad_mode_requires_no_padding_rejects_any_padding < E : EngineWrapper > (engine_wrapper : E) { let engine = E :: standard_with_pad_mode (true , DecodePaddingMode :: RequireNone) ; let suffixes = ["/w=" , "/w==" , "iYU="] ; for num_prefix_quads in 0 .. 256 { for & suffix in suffixes . iter () { let mut encoded = "AAAA" . repeat (num_prefix_quads) ; encoded . push_str (suffix) ; let res = engine . decode (& encoded) ; assert_eq ! (Err (DecodeError :: InvalidPadding) , res) ; } } }
    };
}

decode_pad_mode_requires_no_padding_rejects_any_padding!();