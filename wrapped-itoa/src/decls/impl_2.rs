macro_rules! deps {
    () => {
        Buffer!();
    };
}

macro_rules! impl_2 {
    () => {
        deps!();
        impl Default for Buffer { # [inline] fn default () -> Buffer { Buffer :: new () } }
    };
}

impl_2!()