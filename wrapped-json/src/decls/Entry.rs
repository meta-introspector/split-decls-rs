macro_rules! deps {
    () => {
        Map!();
        VacantEntry!();
        OccupiedEntry!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " A view into a single entry in a map, which may either be vacant or occupied."] # [doc = " This enum is constructed from the [`entry`] method on [`Map`]."] # [doc = ""] # [doc = " [`entry`]: Map::entry"] pub enum Entry < 'a > { # [doc = " A vacant Entry."] Vacant (VacantEntry < 'a >) , # [doc = " An occupied Entry."] Occupied (OccupiedEntry < 'a >) , }
    };
}

Entry!();