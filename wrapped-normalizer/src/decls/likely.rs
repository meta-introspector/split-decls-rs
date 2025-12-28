macro_rules! likely {
    () => {
        # [doc = " No-op for typed trie case."] # [cfg (all (not (icu4x_unstable_fast_trie_only) , feature = "utf16_iter"))] # [inline (always)] fn likely (b : bool) -> bool { b }
    };
}

likely!()