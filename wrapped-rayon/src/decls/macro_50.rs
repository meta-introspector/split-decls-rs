macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! macro_50 {
    () => {
        deps!();
        into_par_vec ! { BTreeSet < T > => IntoIter < T >, impl < T : Send > }
    };
}

macro_50!();