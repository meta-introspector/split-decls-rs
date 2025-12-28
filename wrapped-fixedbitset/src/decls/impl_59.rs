macro_rules! deps {
    () => {
        Block!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        impl Default for Block { # [inline] fn default () -> Self { Self :: NONE } }
    };
}

impl_59!();