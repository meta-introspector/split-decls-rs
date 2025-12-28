macro_rules! deps {
    () => {
        TimeVal!();
    };
}

macro_rules! impl_180 {
    () => {
        deps!();
        impl Ord for TimeVal { fn cmp (& self , other : & TimeVal) -> cmp :: Ordering { if self . tv_sec () == other . tv_sec () { self . tv_usec () . cmp (& other . tv_usec ()) } else { self . tv_sec () . cmp (& other . tv_sec ()) } } }
    };
}

impl_180!();