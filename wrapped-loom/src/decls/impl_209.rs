macro_rules! deps {
    () => {
        Cell!();
    };
}

macro_rules! impl_209 {
    () => {
        deps!();
        impl < T : Ord + Copy > Ord for Cell < T > { fn cmp (& self , other : & Self) -> core :: cmp :: Ordering { self . get () . cmp (& other . get ()) } }
    };
}

impl_209!()