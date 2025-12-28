macro_rules! deps {
    () => {
        OccupiedEntryImpl!();
        Entry!();
    };
}

macro_rules! OccupiedEntry {
    () => {
        deps!();
        # [doc = " An occupied Entry. It is part of the [`Entry`] enum."] pub struct OccupiedEntry < 'a > { occupied : OccupiedEntryImpl < 'a > , }
    };
}

OccupiedEntry!();