macro_rules! deps {
    () => {
        AsFilename!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl AsFilename for & String { }
    };
}

impl_6!();