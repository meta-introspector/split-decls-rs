macro_rules! deps {
    () => {
        Error!();
        Tree!();
        Kind!();
        Object!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl < 'repo > TryFrom < Object < 'repo > > for Tree < 'repo > { type Error = Object < 'repo > ; fn try_from (mut value : Object < 'repo >) -> Result < Self , Self :: Error > { let repo = value . repo ; match value . kind { object :: Kind :: Tree => Ok (Tree { id : value . id , repo , data : steal_from_freelist (& mut value . data) , }) , _ => Err (value) , } } }
    };
}

impl_172!();