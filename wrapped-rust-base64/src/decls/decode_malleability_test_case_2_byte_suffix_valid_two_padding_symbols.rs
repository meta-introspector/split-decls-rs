macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! decode_malleability_test_case_2_byte_suffix_valid_two_padding_symbols {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_malleability_test_case_2_byte_suffix_valid_two_padding_symbols < E : EngineWrapper > (engine_wrapper : E ,) { assert_eq ! (b"Hell" . as_slice () , & E :: standard () . decode ("SGVsbA==") . unwrap ()) ; }
    };
}

decode_malleability_test_case_2_byte_suffix_valid_two_padding_symbols!()