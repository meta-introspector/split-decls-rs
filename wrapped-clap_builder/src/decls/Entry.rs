macro_rules! deps {
    () => {
        VacantEntry!();
        OccupiedEntry!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        pub (crate) enum Entry < 'a , K , V > { Vacant (VacantEntry < 'a , K , V >) , Occupied (OccupiedEntry < 'a , K , V >) , }
    };
}

Entry!();