macro_rules! deps {
    () => {
        Error!();
        Object!();
        Tree!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        impl TryFrom < Object > for Tree { type Error = Object ; fn try_from (value : Object) -> Result < Self , Self :: Error > { Ok (match value { Object :: Tree (v) => v , _ => return Err (value) , }) } }
    };
}

impl_65!();