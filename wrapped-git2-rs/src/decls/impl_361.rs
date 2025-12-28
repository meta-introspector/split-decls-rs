macro_rules! deps {
    () => {
        DiffFormatEmailOptions!();
    };
}

macro_rules! impl_361 {
    () => {
        deps!();
        impl Default for DiffFormatEmailOptions { fn default () -> Self { Self :: new () } }
    };
}

impl_361!();