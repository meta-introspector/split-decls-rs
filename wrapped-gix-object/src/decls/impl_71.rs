macro_rules! deps {
    () => {
        TagRef!();
        Error!();
        Tag!();
        ObjectRef!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl < 'a > TryFrom < ObjectRef < 'a > > for TagRef < 'a > { type Error = ObjectRef < 'a > ; fn try_from (value : ObjectRef < 'a >) -> Result < Self , Self :: Error > { Ok (match value { ObjectRef :: Tag (v) => v , _ => return Err (value) , }) } }
    };
}

impl_71!();