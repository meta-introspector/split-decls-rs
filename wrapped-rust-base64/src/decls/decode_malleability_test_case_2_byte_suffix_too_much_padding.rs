macro_rules! deps {
    () => {
        DecodeError!();
        EngineWrapper!();
    };
}

macro_rules! decode_malleability_test_case_2_byte_suffix_too_much_padding {
    () => {
        deps!();
        # [apply (all_engines_except_decoder_reader)] fn decode_malleability_test_case_2_byte_suffix_too_much_padding < E : EngineWrapper > (engine_wrapper : E ,) { assert_eq ! (DecodeError :: InvalidByte (6 , PAD_BYTE) , E :: standard () . decode ("SGVsbA====") . unwrap_err ()) ; }
    };
}

decode_malleability_test_case_2_byte_suffix_too_much_padding!();