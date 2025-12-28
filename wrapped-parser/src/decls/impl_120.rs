macro_rules! deps {
    () => {
        Pos!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl From < (usize , usize) > for Pos { fn from ((line , column) : (usize , usize)) -> Self { Self { line , column } } }
    };
}

impl_120!()