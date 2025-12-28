macro_rules! deps {
    () => {
        OsStr!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < Cow < 'static , str > > for OsStr { fn from (cow : Cow < 'static , str >) -> Self { match cow { Cow :: Borrowed (s) => Self :: from (s) , Cow :: Owned (s) => Self :: from (s) , } } }
    };
}

impl_118!()