macro_rules! deps {
    () => {
        VacantEntry!();
        OccupiedEntry!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        pub enum Entry < 'a , K , V , S > { Occupied (OccupiedEntry < 'a , K , V , S >) , Vacant (VacantEntry < 'a , K , V , S >) , }
    };
}

Entry!()