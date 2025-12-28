macro_rules! deps {
    () => {
        BasicString!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl Default for BasicString { fn default () -> Self { Self (core :: ptr :: null_mut ()) } }
    };
}

impl_56!();