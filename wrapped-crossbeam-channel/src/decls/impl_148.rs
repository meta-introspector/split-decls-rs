macro_rules! deps {
    () => {
        ZeroToken!();
    };
}

macro_rules! impl_148 {
    () => {
        deps!();
        impl Default for ZeroToken { fn default () -> Self { Self (ptr :: null_mut ()) } }
    };
}

impl_148!()