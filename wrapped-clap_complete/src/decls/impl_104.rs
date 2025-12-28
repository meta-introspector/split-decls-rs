macro_rules! deps {
    () => {
        ArgValueCompleter!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        impl ArgExt for ArgValueCompleter { }
    };
}

impl_104!()