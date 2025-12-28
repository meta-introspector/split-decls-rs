macro_rules! deps {
    () => {
        Decompress!();
    };
}

macro_rules! impl_89 {
    () => {
        deps!();
        impl Default for Decompress { fn default () -> Self { Self :: new () } }
    };
}

impl_89!()