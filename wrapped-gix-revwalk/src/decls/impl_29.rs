macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_29 {
    () => {
        deps!();
        impl < K : Ord , T > PartialEq < Self > for Item < K , T > { fn eq (& self , other : & Self) -> bool { Ord :: cmp (self , other) . is_eq () } }
    };
}

impl_29!()