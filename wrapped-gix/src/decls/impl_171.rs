macro_rules! deps {
    () => {
        Tag!();
        Kind!();
        Error!();
        Object!();
    };
}

macro_rules! impl_171 {
    () => {
        deps!();
        impl < 'repo > TryFrom < Object < 'repo > > for Tag < 'repo > { type Error = Object < 'repo > ; fn try_from (mut value : Object < 'repo >) -> Result < Self , Self :: Error > { let repo = value . repo ; match value . kind { object :: Kind :: Tag => Ok (Tag { id : value . id , repo , data : steal_from_freelist (& mut value . data) , }) , _ => Err (value) , } } }
    };
}

impl_171!()