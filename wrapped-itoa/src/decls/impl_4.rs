macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl Default for Buffer { # [inline] fn default () -> Buffer { Buffer :: new () } }
    };
}

impl_4!()