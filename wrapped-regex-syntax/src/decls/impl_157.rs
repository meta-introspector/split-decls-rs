macro_rules! deps {
    () => {
        Extractor!();
    };
}

macro_rules! impl_157 {
    () => {
        deps!();
        impl Default for Extractor { fn default () -> Extractor { Extractor :: new () } }
    };
}

impl_157!();