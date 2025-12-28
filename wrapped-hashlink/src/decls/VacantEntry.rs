macro_rules! deps {
    () => {
        RawVacantEntryMut!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        pub struct VacantEntry < 'a , K , V , S > { key : K , raw_entry : RawVacantEntryMut < 'a , K , V , S > , }
    };
}

VacantEntry!()