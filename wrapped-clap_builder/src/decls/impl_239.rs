macro_rules! deps {
    () => {
        StyledStr!();
    };
}

macro_rules! impl_239 {
    () => {
        deps!();
        impl From < Cow < 'static , str > > for StyledStr { fn from (cow : Cow < 'static , str >) -> Self { match cow { Cow :: Borrowed (s) => StyledStr :: from (s) , Cow :: Owned (s) => StyledStr :: from (s) , } } }
    };
}

impl_239!();