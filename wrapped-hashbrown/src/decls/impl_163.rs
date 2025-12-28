macro_rules! deps {
    () => {
        RawIter!();
        RawParIter!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl < T > From < RawIter < T > > for RawParIter < T > { fn from (it : RawIter < T >) -> Self { RawParIter { iter : it . iter } } }
    };
}

impl_163!()