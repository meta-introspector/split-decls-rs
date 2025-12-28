macro_rules! deps {
    () => {
        ObjectRef!();
        TreeRef!();
        Tree!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < 'a > From < TreeRef < 'a > > for ObjectRef < 'a > { fn from (v : TreeRef < 'a >) -> Self { ObjectRef :: Tree (v) } }
    };
}

impl_69!()