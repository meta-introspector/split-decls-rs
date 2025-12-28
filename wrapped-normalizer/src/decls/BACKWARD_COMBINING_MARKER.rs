macro_rules! BACKWARD_COMBINING_MARKER {
    () => {
        # [doc = " Marker that the first character of the decomposition"] # [doc = " can combine backwards."] # [doc = ""] # [doc = " See trie-value-format.md"] const BACKWARD_COMBINING_MARKER : u32 = 1 << 31 ;
    };
}

BACKWARD_COMBINING_MARKER!()