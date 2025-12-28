macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl Default for Bytes { # [inline] fn default () -> Bytes { Bytes :: new () } }
    };
}

impl_106!();