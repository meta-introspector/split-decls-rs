macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! macro_83 {
    () => {
        deps!();
        into_par_vec ! { LinkedList < T > => IntoIter < T >, impl < T : Send > }
    };
}

macro_83!();