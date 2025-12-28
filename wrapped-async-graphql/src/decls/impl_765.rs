macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! impl_765 {
    () => {
        deps!();
        impl From < ID > for String { fn from (id : ID) -> Self { id . 0 } }
    };
}

impl_765!();