macro_rules! deps {
    () => {
        ArrayString!();
        CapacityError!();
    };
}

macro_rules! impl_33 {
    () => {
        deps!();
        impl < 'a , const CAP : usize > TryFrom < & 'a str > for ArrayString < CAP > { type Error = CapacityError < & 'a str > ; fn try_from (f : & 'a str) -> Result < Self , Self :: Error > { let mut v = Self :: new () ; v . try_push_str (f) ? ; Ok (v) } }
    };
}

impl_33!();