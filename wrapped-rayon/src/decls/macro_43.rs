macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_43 {
    () => {
        deps!();
        into_par_vec ! { &'a BTreeMap < K , V > => Iter <'a , K , V >, impl <'a , K : Sync , V : Sync > }
    };
}

macro_43!();