macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! macro_62 {
    () => {
        deps!();
        into_par_vec ! { &'a HashMap < K , V , S > => Iter <'a , K , V >, impl <'a , K : Sync , V : Sync , S > }
    };
}

macro_62!()