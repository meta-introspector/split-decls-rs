macro_rules! deps {
    () => {
        SyntaxText!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl Eq for SyntaxText { }
    };
}

impl_96!();