macro_rules! deps {
    () => {
        ExtractKind!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl Default for ExtractKind { fn default () -> ExtractKind { ExtractKind :: Prefix } }
    };
}

impl_160!();