macro_rules! deps {
    () => {
        CopyOptions!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl Default for CopyOptions { fn default () -> Self { CopyOptions :: new () } }
    };
}

impl_14!();