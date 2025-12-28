macro_rules! deps {
    () => {
        Definition!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl From < Impl > for Definition { fn from (impl_ : Impl) -> Self { Definition :: SelfType (impl_) } }
    };
}

impl_35!()