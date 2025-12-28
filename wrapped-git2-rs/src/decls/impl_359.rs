macro_rules! deps {
    () => {
        DiffFindOptions!();
    };
}

macro_rules! impl_359 {
    () => {
        deps!();
        impl Default for DiffFindOptions { fn default () -> Self { Self :: new () } }
    };
}

impl_359!()