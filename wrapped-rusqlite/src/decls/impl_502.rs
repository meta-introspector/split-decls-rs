macro_rules! deps {
    () => {
        Null!();
        ValueRef!();
        Blob!();
        Type!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        impl ValueRef < '_ > { # [doc = " Returns SQLite fundamental datatype."] # [inline] # [must_use] pub fn data_type (& self) -> Type { match * self { ValueRef :: Null => Type :: Null , ValueRef :: Integer (_) => Type :: Integer , ValueRef :: Real (_) => Type :: Real , ValueRef :: Text (_) => Type :: Text , ValueRef :: Blob (_) => Type :: Blob , } } }
    };
}

impl_502!()