macro_rules! deps {
    () => {
        ValueRef!();
        Null!();
        Value!();
        Blob!();
    };
}

macro_rules! impl_507 {
    () => {
        deps!();
        impl < 'a > From < & 'a Value > for ValueRef < 'a > { # [inline] fn from (value : & 'a Value) -> Self { match * value { Value :: Null => ValueRef :: Null , Value :: Integer (i) => ValueRef :: Integer (i) , Value :: Real (r) => ValueRef :: Real (r) , Value :: Text (ref s) => ValueRef :: Text (s . as_bytes ()) , Value :: Blob (ref b) => ValueRef :: Blob (b) , } } }
    };
}

impl_507!()