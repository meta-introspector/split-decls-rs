macro_rules! deps {
    () => {
        AnonConst!();
        Const!();
        Term!();
    };
}

macro_rules! impl_127 {
    () => {
        deps!();
        impl From < AnonConst > for Term { fn from (v : AnonConst) -> Self { Term :: Const (v) } }
    };
}

impl_127!()