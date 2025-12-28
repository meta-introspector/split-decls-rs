macro_rules! deps {
    () => {
        EngineWrapper!();
        DecodeError!();
    };
}

macro_rules! decode_malleability_test_case_2_byte_suffix_short_padding {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_malleability_test_case_2_byte_suffix_short_padding < E : EngineWrapper > (engine_wrapper : E) { assert_eq ! (DecodeError :: InvalidPadding , E :: standard () . decode ("SGVsbA=") . unwrap_err ()) ; }
    };
}

decode_malleability_test_case_2_byte_suffix_short_padding!();