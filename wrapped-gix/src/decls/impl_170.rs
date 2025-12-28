macro_rules! deps {
    () => {
        Error!();
        Kind!();
        Commit!();
        Object!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < 'repo > TryFrom < Object < 'repo > > for Commit < 'repo > { type Error = Object < 'repo > ; fn try_from (mut value : Object < 'repo >) -> Result < Self , Self :: Error > { let repo = value . repo ; match value . kind { object :: Kind :: Commit => Ok (Commit { id : value . id , repo , data : steal_from_freelist (& mut value . data) , }) , _ => Err (value) , } } }
    };
}

impl_170!();