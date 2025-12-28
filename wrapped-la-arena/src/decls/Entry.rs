macro_rules! deps {
    () => {
        ArenaMap!();
        OccupiedEntry!();
        VacantEntry!();
    };
}

macro_rules! Entry {
    () => {
        deps!();
        # [doc = " A view into a single entry in a map, which may either be vacant or occupied."] # [doc = ""] # [doc = " This `enum` is constructed from the [`entry`] method on [`ArenaMap`]."] # [doc = ""] # [doc = " [`entry`]: ArenaMap::entry"] pub enum Entry < 'a , IDX , V > { # [doc = " A vacant entry."] Vacant (VacantEntry < 'a , IDX , V >) , # [doc = " An occupied entry."] Occupied (OccupiedEntry < 'a , IDX , V >) , }
    };
}

Entry!()