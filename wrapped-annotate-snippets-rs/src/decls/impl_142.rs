macro_rules! deps {
    () => {
        Origin!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl < 'a > From < Cow < 'a , str > > for Origin < 'a > { fn from (origin : Cow < 'a , str >) -> Self { Self :: path (origin) } }
    };
}

impl_142!()