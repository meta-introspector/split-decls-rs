macro_rules! deps {
    () => {
        Object!();
        Commit!();
        Error!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl TryFrom < Object > for Commit { type Error = Object ; fn try_from (value : Object) -> Result < Self , Self :: Error > { Ok (match value { Object :: Commit (v) => v , _ => return Err (value) , }) } }
    };
}

impl_64!();