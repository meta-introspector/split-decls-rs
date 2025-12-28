macro_rules! deps {
    () => {
        Tag!();
        Object!();
        Error!();
    };
}

macro_rules! impl_63 {
    () => {
        deps!();
        impl TryFrom < Object > for Tag { type Error = Object ; fn try_from (value : Object) -> Result < Self , Self :: Error > { Ok (match value { Object :: Tag (v) => v , _ => return Err (value) , }) } }
    };
}

impl_63!()