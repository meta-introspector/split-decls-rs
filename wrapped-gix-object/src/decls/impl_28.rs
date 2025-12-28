macro_rules! deps {
    () => {
        SignedData!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < SignedData < '_ > > for BString { fn from (value : SignedData < '_ >) -> Self { value . to_bstring () } }
    };
}

impl_28!()