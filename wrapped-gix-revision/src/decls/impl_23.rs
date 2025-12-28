macro_rules! deps {
    () => {
        GenThenTime!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl Ord for GenThenTime { fn cmp (& self , other : & Self) -> Ordering { self . generation . cmp (& other . generation) . then (self . time . cmp (& other . time)) } }
    };
}

impl_23!();