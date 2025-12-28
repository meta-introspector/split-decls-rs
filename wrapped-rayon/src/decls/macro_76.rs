macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_76 {
    () => {
        deps!();
        into_par_vec ! { &'a HashSet < T , S > => Iter <'a , T >, impl <'a , T : Sync , S > }
    };
}

macro_76!();