// Generated macro for decode_too_little_data_before_padding_error_invalid_byte (function)
macro_rules! Depcrate_engine_testsdecode_too_little_data_before_padding_error_invalid_byte {
() => {
// Module: crate::engine::tests
// Provides: {"decode_too_little_data_before_padding_error_invalid_byte"}
// Dependencies: {}
# [doc = " 0-1 bytes of data before any amount of padding in final chunk = invalid byte, since padding"] # [doc = " is not valid data (consistent with error for pad bytes in earlier chunks)."] # [doc = " From this we know there must be 2-3 bytes of data before padding"] # [apply (all_engines)] fn decode_too_little_data_before_padding_error_invalid_byte < E : EngineWrapper > (engine_wrapper : E) { let mut rng = seeded_rng () ; let prefix_quads_range = distributions :: Uniform :: from (0_usize .. 256) ; let suffix_data_len_range = distributions :: Uniform :: from (0_usize ..= 1) ; for mode in all_pad_modes () { let engine = E :: standard_with_pad_mode (true , mode) ; for _ in 0 .. 100_000 { let suffix_data_len = suffix_data_len_range . sample (& mut rng) ; let prefix_quad_len = prefix_quads_range . sample (& mut rng) ; for padding_len in 1 ..= (4 - suffix_data_len) { let mut encoded = "ABCD" . repeat (prefix_quad_len) . into_bytes () ; encoded . resize (encoded . len () + suffix_data_len , b'A') ; encoded . resize (encoded . len () + padding_len , PAD_BYTE) ; assert_eq ! (Err (DecodeError :: InvalidByte (prefix_quad_len * 4 + suffix_data_len , PAD_BYTE ,)) , engine . decode (& encoded) , "input {} suffix data len {} pad len {}" , String :: from_utf8 (encoded) . unwrap () , suffix_data_len , padding_len) ; } } } }
};
}
