macro_rules! deps {
    () => {
        ID!();
    };
}

macro_rules! impl_766 {
    () => {
        deps!();
        impl From < ID > for ConstValue { fn from (id : ID) -> Self { ConstValue :: String (id . 0) } }
    };
}

impl_766!();