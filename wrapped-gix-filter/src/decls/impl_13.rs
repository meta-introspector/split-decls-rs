macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl Default for Mode { fn default () -> Self { if cfg ! (windows) { Mode :: CrLf } else { Mode :: Lf } } }
    };
}

impl_13!();