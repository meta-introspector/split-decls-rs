macro_rules! deps {
    () => {
        Ty!();
        Term!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl From < Box < Ty > > for Term { fn from (v : Box < Ty >) -> Self { Term :: Ty (v) } }
    };
}

impl_126!();