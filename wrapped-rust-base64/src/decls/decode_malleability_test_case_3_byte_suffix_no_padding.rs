macro_rules! deps {
    () => {
        DecodeError!();
        EngineWrapper!();
    };
}

macro_rules! decode_malleability_test_case_3_byte_suffix_no_padding {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_malleability_test_case_3_byte_suffix_no_padding < E : EngineWrapper > (engine_wrapper : E) { assert_eq ! (DecodeError :: InvalidPadding , E :: standard () . decode ("SGVsbG9") . unwrap_err ()) ; }
    };
}

decode_malleability_test_case_3_byte_suffix_no_padding!();