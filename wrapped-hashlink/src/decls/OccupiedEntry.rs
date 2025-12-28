macro_rules! deps {
    () => {
        RawOccupiedEntryMut!();
    };
}

macro_rules! OccupiedEntry {
    () => {
        deps!();
        pub struct OccupiedEntry < 'a , K , V , S > { key : K , raw_entry : RawOccupiedEntryMut < 'a , K , V , S > , }
    };
}

OccupiedEntry!()