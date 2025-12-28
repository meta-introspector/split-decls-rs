macro_rules! deps {
    () => {
        Term!();
        Ty!();
    };
}

macro_rules! impl_126 {
    () => {
        deps!();
        impl From < Box < Ty > > for Term { fn from (v : Box < Ty >) -> Self { Term :: Ty (v) } }
    };
}

impl_126!()