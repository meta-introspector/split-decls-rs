macro_rules! deps {
    () => {
        ObjectRef!();
        Blob!();
        BlobRef!();
    };
}

macro_rules! impl_70 {
    () => {
        deps!();
        impl < 'a > From < BlobRef < 'a > > for ObjectRef < 'a > { fn from (v : BlobRef < 'a >) -> Self { ObjectRef :: Blob (v) } }
    };
}

impl_70!()