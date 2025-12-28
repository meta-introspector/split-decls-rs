macro_rules! deps {
    () => {
        OccupiedEntry!();
        VacantEntry!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " A view into an entry in the map"] pub enum Entry < 'a , K , V > { # [doc = " The entry corresponding to the key `K` exists in the map"] Occupied (OccupiedEntry < 'a , K , V >) , # [doc = " The entry corresponding to the key `K` does not exist in the map"] Vacant (VacantEntry < 'a , K , V >) , }
    };
}

Entry!();