// Generated macro for starter_and_decomposes_to_self_impl (function)
macro_rules! Depcratestarter_and_decomposes_to_self_impl {
() => {
// Module: crate
// Provides: {"starter_and_decomposes_to_self_impl"}
// Dependencies: {}
# [doc = " See trie-value-format.md"] # [inline (always)] fn starter_and_decomposes_to_self_impl (trie_val : u32) -> bool { (trie_val & ! (BACKWARD_COMBINING_MARKER | NON_ROUND_TRIP_MARKER)) == 0 }
};
}
