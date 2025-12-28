macro_rules! deps {
    () => {
        RegexSet!();
    };
}

macro_rules! impl_138 {
    () => {
        deps!();
        impl Default for RegexSet { fn default () -> Self { RegexSet :: empty () } }
    };
}

impl_138!()