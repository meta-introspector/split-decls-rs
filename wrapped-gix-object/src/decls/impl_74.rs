macro_rules! deps {
    () => {
        BlobRef!();
        ObjectRef!();
        Blob!();
        Error!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl < 'a > TryFrom < ObjectRef < 'a > > for BlobRef < 'a > { type Error = ObjectRef < 'a > ; fn try_from (value : ObjectRef < 'a >) -> Result < Self , Self :: Error > { Ok (match value { ObjectRef :: Blob (v) => v , _ => return Err (value) , }) } }
    };
}

impl_74!();