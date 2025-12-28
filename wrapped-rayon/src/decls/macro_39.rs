macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! macro_39 {
    () => {
        deps!();
        into_par_vec ! { BTreeMap < K , V > => IntoIter < K , V >, impl < K : Send , V : Send > }
    };
}

macro_39!()