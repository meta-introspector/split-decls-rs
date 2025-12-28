macro_rules! deps {
    () => {
        ObjectRef!();
        Tag!();
        TagRef!();
    };
}

macro_rules! impl_67 {
    () => {
        deps!();
        impl < 'a > From < TagRef < 'a > > for ObjectRef < 'a > { fn from (v : TagRef < 'a >) -> Self { ObjectRef :: Tag (v) } }
    };
}

impl_67!();