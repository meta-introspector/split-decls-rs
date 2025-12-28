macro_rules! deps {
    () => {
        ObjectRef!();
        CommitRef!();
        Commit!();
    };
}

macro_rules! impl_68 {
    () => {
        deps!();
        impl < 'a > From < CommitRef < 'a > > for ObjectRef < 'a > { fn from (v : CommitRef < 'a >) -> Self { ObjectRef :: Commit (v) } }
    };
}

impl_68!();