macro_rules! deps {
    () => {
        ExtractKind!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl ExtractKind { # [doc = " Returns true if this kind is the `Prefix` variant."] pub fn is_prefix (& self) -> bool { matches ! (* self , ExtractKind :: Prefix) } # [doc = " Returns true if this kind is the `Suffix` variant."] pub fn is_suffix (& self) -> bool { matches ! (* self , ExtractKind :: Suffix) } }
    };
}

impl_159!();