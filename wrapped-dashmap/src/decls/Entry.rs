macro_rules! deps {
    () => {
        VacantEntry!();
        OccupiedEntry!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        pub enum Entry < 'a , K , V > { Occupied (OccupiedEntry < 'a , K , V >) , Vacant (VacantEntry < 'a , K , V >) , }
    };
}

Entry!();