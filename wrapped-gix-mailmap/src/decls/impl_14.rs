macro_rules! deps {
    () => {
        Signature!();
    };
}

macro_rules! impl_14 {
    () => {
        deps!();
        impl < 'a > From < gix_actor :: SignatureRef < 'a > > for Signature < 'a > { fn from (s : gix_actor :: SignatureRef < 'a >) -> Self { Signature { name : s . name . into () , email : s . email . into () , time : s . time . parse () . unwrap_or_default () , } } }
    };
}

impl_14!()