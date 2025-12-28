macro_rules! deps {
    () => {
        TimeSpec!();
    };
}

macro_rules! impl_163 {
    () => {
        deps!();
        impl Ord for TimeSpec { fn cmp (& self , other : & TimeSpec) -> cmp :: Ordering { if self . tv_sec () == other . tv_sec () { self . tv_nsec () . cmp (& other . tv_nsec ()) } else { self . tv_sec () . cmp (& other . tv_sec ()) } } }
    };
}

impl_163!();