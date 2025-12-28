macro_rules! deps {
    () => {
        LenType!();
        String!();
        CapacityError!();
    };
}

macro_rules! impl_234 {
    () => {
        deps!();
        impl < LenT : LenType , const N : usize > str :: FromStr for String < N , LenT > { type Err = CapacityError ; fn from_str (s : & str) -> Result < Self , Self :: Err > { let mut new = Self :: new () ; new . push_str (s) ? ; Ok (new) } }
    };
}

impl_234!()