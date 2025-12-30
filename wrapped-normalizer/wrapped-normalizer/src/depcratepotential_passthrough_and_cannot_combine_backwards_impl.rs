// Generated macro for potential_passthrough_and_cannot_combine_backwards_impl (function)
macro_rules! Depcratepotential_passthrough_and_cannot_combine_backwards_impl {
() => {
// Module: crate
// Provides: {"potential_passthrough_and_cannot_combine_backwards_impl"}
// Dependencies: {}
# [doc = " See trie-value-format.md"] # [inline (always)] fn potential_passthrough_and_cannot_combine_backwards_impl (trie_val : u32) -> bool { (trie_val & (NON_ROUND_TRIP_MARKER | BACKWARD_COMBINING_MARKER)) == 0 }
};
}
