macro_rules! deps {
    () => {
        LinkedHashMap!();
    };
}

macro_rules! RawEntryBuilder {
    () => {
        deps!();
        pub struct RawEntryBuilder < 'a , K , V , S > { map : & 'a LinkedHashMap < K , V , S > , }
    };
}

RawEntryBuilder!()