macro_rules! deps {
    () => {
        FlatMap!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        pub (crate) struct VacantEntry < 'a , K , V > { v : & 'a mut FlatMap < K , V > , key : K , }
    };
}

VacantEntry!();