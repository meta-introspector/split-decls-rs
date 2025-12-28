macro_rules! deps {
    () => {
        Commit!();
        Object!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl From < Commit > for Object { fn from (v : Commit) -> Self { Object :: Commit (v) } }
    };
}

impl_60!()