macro_rules! deps {
    () => {
        IdentUnraw!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        impl Ord for IdentUnraw { fn cmp (& self , other : & Self) -> Ordering { Ord :: cmp (& self . 0 . unraw () , & other . 0 . unraw ()) } }
    };
}

impl_100!();