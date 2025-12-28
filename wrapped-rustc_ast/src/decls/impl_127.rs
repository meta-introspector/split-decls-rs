macro_rules! deps {
    () => {
        Term!();
        AnonConst!();
        Const!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl From < AnonConst > for Term { fn from (v : AnonConst) -> Self { Term :: Const (v) } }
    };
}

impl_127!();