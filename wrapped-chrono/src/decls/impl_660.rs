macro_rules! deps {
    () => {
        Transition!();
    };
}

macro_rules! impl_660 {
    () => {
        deps!();
        # [cfg (windows)] impl Ord for Transition { fn cmp (& self , other : & Self) -> Ordering { self . transition_utc . cmp (& other . transition_utc) } }
    };
}

impl_660!();