macro_rules! deps {
    () => {
        RawVacantEntryMut!();
        RawOccupiedEntryMut!();
    };
}

macro_rules! RawEntryMut {
    () => {
        deps!();
        pub enum RawEntryMut < 'a , K , V , S > { Occupied (RawOccupiedEntryMut < 'a , K , V , S >) , Vacant (RawVacantEntryMut < 'a , K , V , S >) , }
    };
}

RawEntryMut!()