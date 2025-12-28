macro_rules! deps {
    () => {
        FlatMap!();
    };
}

macro_rules! OccupiedEntry {
    () => {
        deps!();
        pub (crate) struct OccupiedEntry < 'a , K , V > { v : & 'a mut FlatMap < K , V > , index : usize , }
    };
}

OccupiedEntry!();