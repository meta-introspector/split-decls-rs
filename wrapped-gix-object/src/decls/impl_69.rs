macro_rules! deps {
    () => {
        Tree!();
        ObjectRef!();
        TreeRef!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl < 'a > From < TreeRef < 'a > > for ObjectRef < 'a > { fn from (v : TreeRef < 'a >) -> Self { ObjectRef :: Tree (v) } }
    };
}

impl_69!();