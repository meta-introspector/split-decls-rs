macro_rules! deps {
    () => {
        Nonce!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl Default for Nonce { # [inline] fn default () -> Self { Nonce :: new () } }
    };
}

impl_20!()