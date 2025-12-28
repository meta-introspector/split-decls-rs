macro_rules! starter_and_decomposes_to_self_impl {
    () => {
        # [doc = " See trie-value-format.md"] # [inline (always)] fn starter_and_decomposes_to_self_impl (trie_val : u32) -> bool { (trie_val & ! (BACKWARD_COMBINING_MARKER | NON_ROUND_TRIP_MARKER)) == 0 }
    };
}

starter_and_decomposes_to_self_impl!()