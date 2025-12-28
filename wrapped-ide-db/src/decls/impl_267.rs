macro_rules! deps {
    () => {
        MiniCore!();
    };
}

macro_rules! impl_267 {
    () => {
        deps!();
        impl < 'a > Default for MiniCore < 'a > { # [inline] fn default () -> Self { Self :: default () } }
    };
}

impl_267!()