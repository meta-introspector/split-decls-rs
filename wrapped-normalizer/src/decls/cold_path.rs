macro_rules! cold_path {
    () => {
        # [cfg (all (icu4x_unstable_fast_trie_only , feature = "utf16_iter"))] # [inline (always)] # [cold] fn cold_path () { }
    };
}

cold_path!()