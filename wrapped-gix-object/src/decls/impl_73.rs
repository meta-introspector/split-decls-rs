macro_rules! deps {
    () => {
        ObjectRef!();
        TreeRef!();
        Error!();
        Tree!();
    };
}

macro_rules! impl_73 {
    () => {
        deps!();
        impl < 'a > TryFrom < ObjectRef < 'a > > for TreeRef < 'a > { type Error = ObjectRef < 'a > ; fn try_from (value : ObjectRef < 'a >) -> Result < Self , Self :: Error > { Ok (match value { ObjectRef :: Tree (v) => v , _ => return Err (value) , }) } }
    };
}

impl_73!()