macro_rules! deps {
    () => {
        File!();
    };
}

macro_rules! impl_437 {
    () => {
        deps!();
        impl Eq for File { }
    };
}

impl_437!();