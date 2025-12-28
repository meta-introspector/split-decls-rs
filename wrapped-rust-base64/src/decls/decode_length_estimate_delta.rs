macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! decode_length_estimate_delta {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_length_estimate_delta < E : EngineWrapper > (engine_wrapper : E) { for engine in [E :: standard () , E :: standard_unpadded ()] { for & padding in & [true , false] { for orig_len in 0 .. 1000 { let encoded_len = encoded_len (orig_len , padding) . unwrap () ; let decoded_estimate = engine . internal_decoded_len_estimate (encoded_len) . decoded_len_estimate () ; assert ! (decoded_estimate >= orig_len) ; assert ! (decoded_estimate - orig_len < 3 , "estimate: {}, encoded: {}, orig: {}" , decoded_estimate , encoded_len , orig_len) ; } } } }
    };
}

decode_length_estimate_delta!();