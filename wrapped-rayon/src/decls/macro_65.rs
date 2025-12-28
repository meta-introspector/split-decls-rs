macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! macro_65 {
    () => {
        deps!();
        into_par_vec ! { &'a mut HashMap < K , V , S > => IterMut <'a , K , V >, impl <'a , K : Sync , V : Send , S > }
    };
}

macro_65!();