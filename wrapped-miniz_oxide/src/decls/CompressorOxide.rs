macro_rules! deps {
    () => {
        HuffmanOxide!();
        ParamsOxide!();
        LZOxide!();
        DictOxide!();
    };
}

macro_rules! CompressorOxide {
    () => {
        deps!();
        # [doc = " Main compression struct."] pub struct CompressorOxide { pub (crate) lz : LZOxide , pub (crate) params : ParamsOxide , # [doc = " Put HuffmanOxide on the heap with default trick to avoid"] # [doc = " excessive stack copies."] pub (crate) huff : Box < HuffmanOxide > , pub (crate) dict : DictOxide , }
    };
}

CompressorOxide!();