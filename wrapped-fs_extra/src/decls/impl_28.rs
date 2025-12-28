macro_rules! deps {
    () => {
        DirOptions!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl DirOptions { # [doc = " Initialize struct DirOptions with default value."] pub fn new () -> DirOptions { Default :: default () } }
    };
}

impl_28!()