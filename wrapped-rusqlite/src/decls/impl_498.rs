macro_rules! deps {
    () => {
        Null!();
        Type!();
        Blob!();
        Value!();
    };
}

macro_rules! impl_498 {
    () => {
        deps!();
        impl Value { # [doc = " Returns SQLite fundamental datatype."] # [inline] # [must_use] pub fn data_type (& self) -> Type { match * self { Self :: Null => Type :: Null , Self :: Integer (_) => Type :: Integer , Self :: Real (_) => Type :: Real , Self :: Text (_) => Type :: Text , Self :: Blob (_) => Type :: Blob , } } }
    };
}

impl_498!();