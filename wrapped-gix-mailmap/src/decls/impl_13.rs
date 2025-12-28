macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl < 'a > From < Signature < 'a > > for gix_actor :: Signature { fn from (s : Signature < 'a >) -> Self { gix_actor :: Signature { name : s . name . into_owned () , email : s . email . into_owned () , time : s . time , } } }
    };
}

impl_13!()