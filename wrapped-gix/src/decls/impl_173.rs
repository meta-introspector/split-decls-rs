macro_rules! deps {
    () => {
        Object!();
        Kind!();
        Error!();
        Blob!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl < 'repo > TryFrom < Object < 'repo > > for Blob < 'repo > { type Error = Object < 'repo > ; fn try_from (mut value : Object < 'repo >) -> Result < Self , Self :: Error > { let repo = value . repo ; match value . kind { object :: Kind :: Blob => Ok (Blob { id : value . id , repo , data : steal_from_freelist (& mut value . data) , }) , _ => Err (value) , } } }
    };
}

impl_173!()