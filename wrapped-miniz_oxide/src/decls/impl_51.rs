macro_rules! deps {
    () => {
        CompressorOxide!();
        LZOxide!();
        DictOxide!();
        ParamsOxide!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Default for CompressorOxide { # [doc = " Initialize the compressor with a level of 4, zlib wrapper and"] # [doc = " the default strategy."] fn default () -> Self { CompressorOxide { lz : LZOxide :: new () , params : ParamsOxide :: new (DEFAULT_FLAGS) , huff : Box :: default () , dict : DictOxide :: new (DEFAULT_FLAGS) , } } }
    };
}

impl_51!();