macro_rules! deps {
    () => {
        Md5Core!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl Reset for Md5Core { # [inline] fn reset (& mut self) { * self = Default :: default () ; } }
    };
}

impl_9!();