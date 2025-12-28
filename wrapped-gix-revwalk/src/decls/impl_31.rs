macro_rules! deps {
    () => {
        Item!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < K : Ord , T > PartialOrd < Self > for Item < K , T > { fn partial_cmp (& self , other : & Self) -> Option < Ordering > { Some (Ord :: cmp (self , other)) } }
    };
}

impl_31!();