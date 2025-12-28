macro_rules! deps {
    () => {
        FullName!();
        Target!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl From < FullName > for Target { fn from (name : FullName) -> Self { Target :: Symbolic (name) } }
    };
}

impl_74!()