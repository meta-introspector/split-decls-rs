macro_rules! deps {
    () => {
        RegexSet!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl Default for RegexSet { fn default () -> Self { RegexSet :: empty () } }
    };
}

impl_155!()