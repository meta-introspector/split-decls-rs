macro_rules! deps {
    () => {
        Utf8Sequences!();
    };
}

macro_rules! impl_873 {
    () => {
        deps!();
        impl FusedIterator for Utf8Sequences { }
    };
}

impl_873!();