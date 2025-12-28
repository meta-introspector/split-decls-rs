macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl Default for Context { # [inline] fn default () -> Self { Self :: new () } }
    };
}

impl_11!()