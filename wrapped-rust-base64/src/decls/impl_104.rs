macro_rules! deps {
    () => {
        Alphabet!();
        GeneralPurpose!();
        GeneralPurposeConfig!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl GeneralPurpose { # [doc = " Create a `GeneralPurpose` engine from an [Alphabet]."] # [doc = ""] # [doc = " While not very expensive to initialize, ideally these should be cached"] # [doc = " if the engine will be used repeatedly."] # [must_use] pub const fn new (alphabet : & Alphabet , config : GeneralPurposeConfig) -> Self { Self { encode_table : encode_table (alphabet) , decode_table : decode_table (alphabet) , config , } } }
    };
}

impl_104!();