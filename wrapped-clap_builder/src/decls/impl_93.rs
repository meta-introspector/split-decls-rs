macro_rules! deps {
    () => {
        AppExt!();
        TermWidth!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl AppExt for TermWidth { }
    };
}

impl_93!();