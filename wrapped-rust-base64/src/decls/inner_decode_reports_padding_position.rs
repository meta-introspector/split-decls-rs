macro_rules! deps {
    () => {
        DecodeSliceError!();
        DecodeMetadata!();
        EngineWrapper!();
        DecodeError!();
    };
}

macro_rules! inner_decode_reports_padding_position {
    () => {
        deps!();
        # [apply (all_engines)] fn inner_decode_reports_padding_position < E : EngineWrapper > (engine_wrapper : E) { let mut b64 = String :: new () ; let mut decoded = Vec :: new () ; let engine = E :: standard () ; for pad_position in 1 .. 10_000 { b64 . clear () ; decoded . clear () ; decoded . resize (pad_position , 0) ; for _ in 0 .. pad_position { b64 . push ('A') ; } for _ in 0 .. (4 - (pad_position % 4)) { b64 . push ('=') ; } let decode_res = engine . internal_decode (b64 . as_bytes () , & mut decoded [..] , engine . internal_decoded_len_estimate (b64 . len ()) ,) ; if pad_position % 4 < 2 { assert_eq ! (Err (DecodeSliceError :: DecodeError (DecodeError :: InvalidByte (pad_position , PAD_BYTE))) , decode_res) ; } else { let decoded_bytes = pad_position / 4 * 3 + match pad_position % 4 { 0 => 0 , 2 => 1 , 3 => 2 , _ => unreachable ! () , } ; assert_eq ! (Ok (DecodeMetadata :: new (decoded_bytes , Some (pad_position))) , decode_res) ; } } }
    };
}

inner_decode_reports_padding_position!()