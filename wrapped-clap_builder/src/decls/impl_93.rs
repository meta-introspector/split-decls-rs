macro_rules! deps {
    () => {
        TermWidth!();
        AppExt!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl AppExt for TermWidth { }
    };
}

impl_93!()