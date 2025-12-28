macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! macro_72 {
    () => {
        deps!();
        into_par_vec ! { HashSet < T , S > => IntoIter < T >, impl < T : Send , S > }
    };
}

macro_72!();