macro_rules! deps {
    () => {
        Node!();
    };
}

macro_rules! RawVacantEntryMut {
    () => {
        deps!();
        pub struct RawVacantEntryMut < 'a , K , V , S > { hash_builder : & 'a S , values : & 'a mut Option < NonNull < Node < K , V > > > , free : & 'a mut Option < NonNull < Node < K , V > > > , entry : hash_table :: AbsentEntry < 'a , NonNull < Node < K , V > > > , }
    };
}

RawVacantEntryMut!()