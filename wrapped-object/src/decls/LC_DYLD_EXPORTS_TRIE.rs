macro_rules! deps {
    () => {
        LinkeditDataCommand!();
    };
}

macro_rules! LC_DYLD_EXPORTS_TRIE {
    () => {
        deps!();
        # [doc = " used with `LinkeditDataCommand`, payload is trie"] pub const LC_DYLD_EXPORTS_TRIE : u32 = 0x33 | LC_REQ_DYLD ;
    };
}

LC_DYLD_EXPORTS_TRIE!();