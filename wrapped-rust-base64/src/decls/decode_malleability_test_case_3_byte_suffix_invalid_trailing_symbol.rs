macro_rules! deps {
    () => {
        EngineWrapper!();
        DecodeError!();
    };
}

macro_rules! decode_malleability_test_case_3_byte_suffix_invalid_trailing_symbol {
    () => {
        deps!();
        # [apply (all_engines)] fn decode_malleability_test_case_3_byte_suffix_invalid_trailing_symbol < E : EngineWrapper > (engine_wrapper : E ,) { assert_eq ! (DecodeError :: InvalidLastSymbol (6 , 0x39) , E :: standard () . decode ("SGVsbG9=") . unwrap_err ()) ; }
    };
}

decode_malleability_test_case_3_byte_suffix_invalid_trailing_symbol!();