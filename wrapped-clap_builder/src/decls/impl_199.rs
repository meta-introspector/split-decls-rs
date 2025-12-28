macro_rules! deps {
    () => {
        Str!();
    };
}

macro_rules! impl_199 {
    () => {
        deps!();
        # [cfg (feature = "string")] impl From < Cow < 'static , str > > for Str { fn from (cow : Cow < 'static , str >) -> Self { match cow { Cow :: Borrowed (s) => Self :: from (s) , Cow :: Owned (s) => Self :: from (s) , } } }
    };
}

impl_199!()