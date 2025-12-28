macro_rules! deps {
    () => {
        Object!();
        Blob!();
    };
}

macro_rules! impl_62 {
    () => {
        deps!();
        impl From < Blob > for Object { fn from (v : Blob) -> Self { Object :: Blob (v) } }
    };
}

impl_62!();