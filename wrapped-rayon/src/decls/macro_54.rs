macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_54 {
    () => {
        deps!();
        into_par_vec ! { &'a BTreeSet < T > => Iter <'a , T >, impl <'a , T : Sync > }
    };
}

macro_54!()