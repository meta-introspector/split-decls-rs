macro_rules! deps {
    () => {
        AsFilename!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl AsFilename for String { }
    };
}

impl_8!();