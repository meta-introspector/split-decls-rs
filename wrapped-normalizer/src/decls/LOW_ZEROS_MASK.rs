macro_rules! LOW_ZEROS_MASK {
    () => {
        # [doc = " Mask for the bits have to be zero for this to be a complex"] # [doc = " decomposition."] # [doc = ""] # [doc = " See trie-value-format.md"] const LOW_ZEROS_MASK : u32 = 0xFFE0 ;
    };
}

LOW_ZEROS_MASK!();