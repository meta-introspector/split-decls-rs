macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_87 {
    () => {
        deps!();
        into_par_vec ! { &'a LinkedList < T > => Iter <'a , T >, impl <'a , T : Sync > }
    };
}

macro_87!()