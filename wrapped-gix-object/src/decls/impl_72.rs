macro_rules! deps {
    () => {
        Error!();
        CommitRef!();
        Commit!();
        ObjectRef!();
    };
}

macro_rules! impl_72 {
    () => {
        deps!();
        impl < 'a > TryFrom < ObjectRef < 'a > > for CommitRef < 'a > { type Error = ObjectRef < 'a > ; fn try_from (value : ObjectRef < 'a >) -> Result < Self , Self :: Error > { Ok (match value { ObjectRef :: Commit (v) => v , _ => return Err (value) , }) } }
    };
}

impl_72!()