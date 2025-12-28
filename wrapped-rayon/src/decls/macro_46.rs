macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! macro_46 {
    () => {
        deps!();
        into_par_vec ! { &'a mut BTreeMap < K , V > => IterMut <'a , K , V >, impl <'a , K : Sync , V : Send > }
    };
}

macro_46!()