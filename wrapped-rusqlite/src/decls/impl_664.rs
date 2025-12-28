macro_rules! deps {
    () => {
        SmallCString!();
    };
}

macro_rules! impl_664 {
    () => {
        deps!();
        impl Default for SmallCString { # [inline] fn default () -> Self { Self (SmallVec :: from_buf ([0])) } }
    };
}

impl_664!()