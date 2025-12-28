macro_rules! deps {
    () => {
        AsFilename!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl AsFilename for & str { }
    };
}

impl_4!();