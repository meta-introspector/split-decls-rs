macro_rules! deps {
    () => {
        Blob!();
        Object!();
        Error!();
    };
}

macro_rules! impl_66 {
    () => {
        deps!();
        impl TryFrom < Object > for Blob { type Error = Object ; fn try_from (value : Object) -> Result < Self , Self :: Error > { Ok (match value { Object :: Blob (v) => v , _ => return Err (value) , }) } }
    };
}

impl_66!();