macro_rules! deps {
    () => {
        PathCompleter!();
    };
}

macro_rules! impl_119 {
    () => {
        deps!();
        impl Default for PathCompleter { fn default () -> Self { Self :: any () } }
    };
}

impl_119!();