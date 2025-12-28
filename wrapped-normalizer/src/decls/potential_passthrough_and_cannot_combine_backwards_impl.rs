macro_rules! potential_passthrough_and_cannot_combine_backwards_impl {
    () => {
        # [doc = " See trie-value-format.md"] # [inline (always)] fn potential_passthrough_and_cannot_combine_backwards_impl (trie_val : u32) -> bool { (trie_val & (NON_ROUND_TRIP_MARKER | BACKWARD_COMBINING_MARKER)) == 0 }
    };
}

potential_passthrough_and_cannot_combine_backwards_impl!();