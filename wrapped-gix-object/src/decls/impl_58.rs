macro_rules! deps {
    () => {
        Commit!();
        Error!();
        Object!();
        ObjectRef!();
        Tree!();
        Blob!();
        Tag!();
    };
}

macro_rules! impl_58 {
    () => {
        deps!();
        impl TryFrom < ObjectRef < '_ > > for Object { type Error = crate :: decode :: Error ; fn try_from (v : ObjectRef < '_ >) -> Result < Self , Self :: Error > { Ok (match v { ObjectRef :: Tree (v) => Object :: Tree (v . into ()) , ObjectRef :: Blob (v) => Object :: Blob (v . into ()) , ObjectRef :: Commit (v) => Object :: Commit (v . try_into () ?) , ObjectRef :: Tag (v) => Object :: Tag (v . try_into () ?) , }) } }
    };
}

impl_58!();