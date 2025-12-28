macro_rules! deps {
    () => {
        BlobRef!();
        Blob!();
    };
}

macro_rules! impl_54 {
    () => {
        deps!();
        impl < 'a > From < BlobRef < 'a > > for Blob { fn from (v : BlobRef < 'a >) -> Self { Blob { data : v . data . to_owned () , } } }
    };
}

impl_54!();