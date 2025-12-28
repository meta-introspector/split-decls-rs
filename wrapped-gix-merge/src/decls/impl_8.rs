macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl Default for Options { fn default () -> Self { Options { conflict : Default :: default () , diff_algorithm : imara_diff :: Algorithm :: Myers , } } }
    };
}

impl_8!();