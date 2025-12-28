macro_rules! deps {
    () => {
        MergeFileOptions!();
    };
}

macro_rules! impl_429 {
    () => {
        deps!();
        impl Default for MergeFileOptions { fn default () -> Self { Self :: new () } }
    };
}

impl_429!();