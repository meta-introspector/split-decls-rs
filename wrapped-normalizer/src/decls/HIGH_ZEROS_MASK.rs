macro_rules! HIGH_ZEROS_MASK {
    () => {
        # [doc = " Mask for the bits have to be zero for this to be a BMP"] # [doc = " singleton decomposition, or value baked into the surrogate"] # [doc = " range."] # [doc = ""] # [doc = " See trie-value-format.md"] const HIGH_ZEROS_MASK : u32 = 0x3FFF0000 ;
    };
}

HIGH_ZEROS_MASK!();