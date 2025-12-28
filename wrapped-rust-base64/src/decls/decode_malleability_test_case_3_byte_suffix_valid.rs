macro_rules! deps {
    () => {
        EngineWrapper!();
    };
}

macro_rules! decode_malleability_test_case_3_byte_suffix_valid {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_malleability_test_case_3_byte_suffix_valid < E : EngineWrapper > (engine_wrapper : E) { assert_eq ! (b"Hello" . as_slice () , & E :: standard () . decode ("SGVsbG8=") . unwrap ()) ; }
    };
}

decode_malleability_test_case_3_byte_suffix_valid!();