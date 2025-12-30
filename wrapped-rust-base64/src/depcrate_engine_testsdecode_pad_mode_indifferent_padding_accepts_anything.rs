// Generated macro for decode_pad_mode_indifferent_padding_accepts_anything (function)
macro_rules! Depcrate_engine_testsdecode_pad_mode_indifferent_padding_accepts_anything {
() => {
// Module: crate::engine::tests
// Provides: {"decode_pad_mode_indifferent_padding_accepts_anything"}
// Dependencies: {}
# [doc = " Indifferent padding accepts 2 + 0-2, 3 + 0-1, 4 + 0 final chunk configuration"] # [apply (all_engines)] fn decode_pad_mode_indifferent_padding_accepts_anything < E : EngineWrapper > (engine_wrapper : E) { assert_all_suffixes_ok (E :: standard_with_pad_mode (true , DecodePaddingMode :: Indifferent) , vec ! ["/w" , "/w=" , "/w==" , "iYU" , "iYU=" , "AAAA"] ,) ; }
};
}
