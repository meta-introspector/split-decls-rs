// Generated macro for decode_pad_mode_requires_canonical_rejects_non_canonical (function)
macro_rules! Depcrate_engine_testsdecode_pad_mode_requires_canonical_rejects_non_canonical {
() => {
// Module: crate::engine::tests
// Provides: {"decode_pad_mode_requires_canonical_rejects_non_canonical"}
// Dependencies: {}
# [doc = " Requires canonical padding -> rejects 2 + 0-1, 3 + 0 final chunk configurations"] # [apply (all_engines)] fn decode_pad_mode_requires_canonical_rejects_non_canonical < E : EngineWrapper > (engine_wrapper : E) { let engine = E :: standard_with_pad_mode (true , DecodePaddingMode :: RequireCanonical) ; let suffixes = ["/w" , "/w=" , "iYU"] ; for num_prefix_quads in 0 .. 256 { for & suffix in suffixes . iter () { let mut encoded = "AAAA" . repeat (num_prefix_quads) ; encoded . push_str (suffix) ; let res = engine . decode (& encoded) ; assert_eq ! (Err (DecodeError :: InvalidPadding) , res) ; } } }
};
}
