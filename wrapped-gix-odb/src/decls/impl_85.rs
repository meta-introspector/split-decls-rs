macro_rules! deps {
    () => {
        Either!();
        Ordering!();
    };
}

macro_rules! impl_85 {
    () => {
        deps!();
        impl Ord for Either { fn cmp (& self , other : & Self) -> std :: cmp :: Ordering { self . path () . cmp (other . path ()) } }
    };
}

impl_85!()