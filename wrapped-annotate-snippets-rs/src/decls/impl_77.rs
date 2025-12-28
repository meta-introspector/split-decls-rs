macro_rules! deps {
    () => {
        Stylesheet!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl Stylesheet { pub (crate) const fn plain () -> Self { Self { error : Style :: new () , warning : Style :: new () , info : Style :: new () , note : Style :: new () , help : Style :: new () , line_num : Style :: new () , emphasis : Style :: new () , none : Style :: new () , context : Style :: new () , addition : Style :: new () , removal : Style :: new () , } } }
    };
}

impl_77!()