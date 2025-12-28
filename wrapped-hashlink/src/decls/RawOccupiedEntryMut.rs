macro_rules! deps {
    () => {
        OccupiedEntry!();
        Node!();
    };
}

macro_rules! RawOccupiedEntryMut {
    () => {
        deps!();
        pub struct RawOccupiedEntryMut < 'a , K , V , S > { hash_builder : & 'a S , free : & 'a mut Option < NonNull < Node < K , V > > > , values : & 'a mut Option < NonNull < Node < K , V > > > , entry : hash_table :: OccupiedEntry < 'a , NonNull < Node < K , V > > > , }
    };
}

RawOccupiedEntryMut!();