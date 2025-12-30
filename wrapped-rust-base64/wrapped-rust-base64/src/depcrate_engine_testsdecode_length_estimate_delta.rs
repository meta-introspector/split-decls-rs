// Generated macro for decode_length_estimate_delta (function)
macro_rules! Depcrate_engine_testsdecode_length_estimate_delta {
() => {
// Module: crate::engine::tests
// Provides: {"decode_length_estimate_delta"}
// Dependencies: {}
# [apply (all_engines)] fn decode_length_estimate_delta < E : EngineWrapper > (engine_wrapper : E) { for engine in [E :: standard () , E :: standard_unpadded ()] { for & padding in & [true , false] { for orig_len in 0 .. 1000 { let encoded_len = encoded_len (orig_len , padding) . unwrap () ; let decoded_estimate = engine . internal_decoded_len_estimate (encoded_len) . decoded_len_estimate () ; assert ! (decoded_estimate >= orig_len) ; assert ! (decoded_estimate - orig_len < 3 , "estimate: {}, encoded: {}, orig: {}" , decoded_estimate , encoded_len , orig_len) ; } } } }
};
}
