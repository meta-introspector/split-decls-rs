macro_rules! deps {
    () => {
        ObjectRef!();
        BlobRef!();
        Blob!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < 'a > From < BlobRef < 'a > > for ObjectRef < 'a > { fn from (v : BlobRef < 'a >) -> Self { ObjectRef :: Blob (v) } }
    };
}

impl_70!();