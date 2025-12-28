macro_rules! deps {
    () => {
        Entry!();
        VacantEntryRef!();
        OccupiedEntryRef!();
    };
}

macro_rules! EntryRef {
    () => {
        deps!();
        # [doc = " Entry with a borrowed key."] pub enum EntryRef < 'a , 'q , K , Q , V > { Occupied (OccupiedEntryRef < 'a , 'q , K , Q , V >) , Vacant (VacantEntryRef < 'a , 'q , K , Q , V >) , }
    };
}

EntryRef!();