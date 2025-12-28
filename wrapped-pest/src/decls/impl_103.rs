macro_rules! deps {
    () => {
        BorrowedOrArc!();
    };
}

macro_rules! impl_103 {
    () => {
        deps!();
        impl From < Cow < 'static , str > > for BorrowedOrArc < '_ > { fn from (value : Cow < 'static , str >) -> Self { match value { Cow :: Borrowed (s) => Self :: Borrowed (s) , Cow :: Owned (s) => Self :: Owned (Arc :: new (s)) , } } }
    };
}

impl_103!();