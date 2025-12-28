macro_rules! deps {
    () => {
        IterMut!();
    };
}

macro_rules! macro_90 {
    () => {
        deps!();
        into_par_vec ! { &'a mut LinkedList < T > => IterMut <'a , T >, impl <'a , T : Send > }
    };
}

macro_90!()