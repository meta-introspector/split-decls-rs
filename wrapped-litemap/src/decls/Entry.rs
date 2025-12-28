macro_rules! deps {
    () => {
        OccupiedEntry!();
        VacantEntry!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " An entry in a `LiteMap`, which may be either occupied or vacant."] # [allow (clippy :: exhaustive_enums)] pub enum Entry < 'a , K , V , S > { Occupied (OccupiedEntry < 'a , K , V , S >) , Vacant (VacantEntry < 'a , K , V , S >) , }
    };
}

Entry!();