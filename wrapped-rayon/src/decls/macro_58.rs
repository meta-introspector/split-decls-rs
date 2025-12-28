macro_rules! deps {
    () => {
        IntoIter!();
    };
}

macro_rules! macro_58 {
    () => {
        deps!();
        into_par_vec ! { HashMap < K , V , S > => IntoIter < K , V >, impl < K : Send , V : Send , S > }
    };
}

macro_58!();