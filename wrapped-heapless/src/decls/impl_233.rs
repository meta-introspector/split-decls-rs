macro_rules! deps {
    () => {
        LenType!();
        String!();
        CapacityError!();
    };
}

macro_rules! impl_233 {
    () => {
        deps!();
        impl < 'a , LenT : LenType , const N : usize > TryFrom < & 'a str > for String < N , LenT > { type Error = CapacityError ; fn try_from (s : & 'a str) -> Result < Self , Self :: Error > { let mut new = Self :: new () ; new . push_str (s) ? ; Ok (new) } }
    };
}

impl_233!();