macro_rules! deps {
    () => {
        Object!();
        Commit!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl From < Commit > for Object { fn from (v : Commit) -> Self { Object :: Commit (v) } }
    };
}

impl_60!();